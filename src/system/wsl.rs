//! WSL / WSL2 process monitoring (issue #14).
//!
//! Windows only exposes WSL as a single `vmmem` / `wslhost` process, so the
//! Linux processes inside a distribution are invisible to the normal process
//! list. This module enumerates the running distributions with
//! `wsl.exe --list --running --quiet` and, for each one, runs a small POSIX
//! shell + awk script (piped in over stdin, so no quoting games) that reads
//! `/proc` directly. That works on every distro that has `sh` and `awk`,
//! including busybox-based ones such as Alpine and the Docker Desktop distros.
//!
//! Sampling happens on a background thread so that the (slow, 100-500ms)
//! `wsl.exe` round-trips never block the TUI. The thread only runs while the
//! WSL tab is active, so users who never open it pay nothing.
//!
//! CPU% is computed from `utime + stime` deltas between two samples, exactly
//! like htop does on Linux (100% = one core fully busy). The very first sample
//! of a process falls back to its lifetime average since there is no delta yet.

use std::collections::HashMap;
use std::io::Write;
use std::os::windows::process::CommandExt;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

const CREATE_NO_WINDOW: u32 = 0x0800_0000;

/// How often the background thread re-samples while the WSL tab is visible.
const SAMPLE_INTERVAL: Duration = Duration::from_millis(2000);
/// How long the thread naps between checks while the tab is not visible.
const IDLE_POLL: Duration = Duration::from_millis(250);
/// When WSL is unavailable (not installed, broken, no distros) re-check this
/// often instead of every sample: a hung wsl.exe should not be respawned all
/// the time.
const UNAVAILABLE_RETRY: Duration = Duration::from_secs(15);
/// Hard cap on any single wsl.exe round-trip. A broken WSL install can make
/// wsl.exe hang for a minute before failing; the tab must not wait for that.
const WSL_TIMEOUT: Duration = Duration::from_secs(10);

// ─── Types ───────────────────────────────────────────────────────────────────

/// One Linux process inside a WSL distribution.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WslProcessInfo {
    pub distro: String,
    pub pid: u32,
    pub ppid: u32,
    pub user: String,
    /// Linux state letter: R, S, D, Z, T, I, ...
    pub state: char,
    pub threads: u32,
    pub cpu_usage: f32,   // percent of one core (htop semantics; can exceed 100)
    pub mem_usage: f32,   // percent of the distro's MemTotal
    pub resident_mem: u64, // bytes
    pub virtual_mem: u64,  // bytes
    pub cpu_time_secs: f64,
    pub name: String,     // comm (kernel thread name / executable basename)
    pub command: String,  // full command line, or "[comm]" for kernel threads
}

/// Availability of WSL on this machine, as seen from the last sample.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WslStatus {
    /// No sample has completed yet.
    Pending,
    /// At least one distribution answered.
    Ok,
    /// `wsl.exe` ran but reported no running distributions.
    NoRunningDistros,
    /// `wsl.exe` could not be launched or failed (message for the user).
    Unavailable(String),
}

#[derive(Debug, Clone)]
pub struct WslSnapshot {
    pub status: WslStatus,
    pub processes: Vec<WslProcessInfo>,
    pub distros: Vec<String>,
    pub sampled_at: Option<Instant>,
}

impl Default for WslSnapshot {
    fn default() -> Self {
        WslSnapshot {
            status: WslStatus::Pending,
            processes: Vec::new(),
            distros: Vec::new(),
            sampled_at: None,
        }
    }
}

// ─── Background collector ────────────────────────────────────────────────────

/// Owns the background sampling thread. Cheap to construct; the thread is only
/// spawned the first time the WSL tab is activated.
pub struct WslCollector {
    active: Arc<AtomicBool>,
    snapshot: Arc<Mutex<WslSnapshot>>,
    thread_started: bool,
}

impl WslCollector {
    pub fn new() -> Self {
        WslCollector {
            active: Arc::new(AtomicBool::new(false)),
            snapshot: Arc::new(Mutex::new(WslSnapshot::default())),
            thread_started: false,
        }
    }

    /// Tell the collector whether the WSL tab is currently visible.
    /// Starts the sampling thread on the first activation.
    pub fn set_active(&mut self, active: bool) {
        self.active.store(active, Ordering::Relaxed);
        if active && !self.thread_started {
            self.thread_started = true;
            let flag = Arc::clone(&self.active);
            let snap = Arc::clone(&self.snapshot);
            std::thread::Builder::new()
                .name("pstop-wsl".into())
                .spawn(move || sampling_loop(flag, snap))
                .ok();
        }
    }

    /// Latest snapshot (clone; the lock is held only for the copy).
    pub fn snapshot(&self) -> WslSnapshot {
        self.snapshot.lock().map(|s| s.clone()).unwrap_or_default()
    }
}

/// Per-process bookkeeping for CPU% deltas, keyed by (distro, pid, starttime)
/// so that a recycled PID never inherits the previous owner's counters.
#[derive(Clone, Copy)]
pub struct PrevSample {
    ticks: u64,
    uptime: f64,
}

fn sampling_loop(active: Arc<AtomicBool>, snapshot: Arc<Mutex<WslSnapshot>>) {
    let mut prev: HashMap<(String, u32, u64), PrevSample> = HashMap::new();

    loop {
        if !active.load(Ordering::Relaxed) {
            std::thread::sleep(IDLE_POLL);
            continue;
        }

        let started = Instant::now();
        let result = sample_once(&mut prev);
        let status = result.status.clone();
        if let Ok(mut s) = snapshot.lock() {
            *s = result;
        }

        // Keep a steady cadence, but stay responsive to deactivation.
        let interval = if matches!(status, WslStatus::Ok) { SAMPLE_INTERVAL } else { UNAVAILABLE_RETRY };
        let elapsed = started.elapsed();
        let mut remaining = interval.saturating_sub(elapsed);
        while remaining > Duration::ZERO && active.load(Ordering::Relaxed) {
            let nap = remaining.min(IDLE_POLL);
            std::thread::sleep(nap);
            remaining = remaining.saturating_sub(nap);
        }
    }
}

fn sample_once(prev: &mut HashMap<(String, u32, u64), PrevSample>) -> WslSnapshot {
    let distros = match list_running_distros() {
        Ok(d) => d,
        Err(msg) => {
            return WslSnapshot {
                status: WslStatus::Unavailable(msg),
                processes: Vec::new(),
                distros: Vec::new(),
                sampled_at: Some(Instant::now()),
            };
        }
    };

    if distros.is_empty() {
        prev.clear();
        return WslSnapshot {
            status: WslStatus::NoRunningDistros,
            processes: Vec::new(),
            distros,
            sampled_at: Some(Instant::now()),
        };
    }

    let mut all = Vec::new();
    let mut seen: std::collections::HashSet<(String, u32, u64)> = std::collections::HashSet::new();
    let mut any_ok = false;
    let mut last_err = String::new();

    for distro in &distros {
        match run_proc_script(distro) {
            Ok(text) => {
                any_ok = true;
                let procs = parse_script_output(distro, &text, prev, &mut seen);
                all.extend(procs);
            }
            Err(e) => last_err = format!("{}: {}", distro, e),
        }
    }

    // Forget processes that disappeared so the map cannot grow forever.
    prev.retain(|k, _| seen.contains(k));

    let status = if any_ok {
        WslStatus::Ok
    } else {
        WslStatus::Unavailable(last_err)
    };

    WslSnapshot {
        status,
        processes: all,
        distros,
        sampled_at: Some(Instant::now()),
    }
}

// ─── wsl.exe plumbing ────────────────────────────────────────────────────────

/// `wsl.exe --list --running --quiet`, decoded from its UTF-16LE output.
fn list_running_distros() -> Result<Vec<String>, String> {
    let mut cmd = Command::new("wsl.exe");
    cmd.args(["--list", "--running", "--quiet"]);
    let output = run_with_timeout(cmd, None, WSL_TIMEOUT)?;

    let stdout = decode_wsl_text(&output.stdout);
    if !output.status.success() {
        let stderr = decode_wsl_text(&output.stderr);
        let msg = first_meaningful_line(&stderr)
            .or_else(|| first_meaningful_line(&stdout))
            .unwrap_or_else(|| format!("wsl.exe exited with {}", output.status));
        // "There are no running distributions" style messages come back with a
        // non-zero exit code on some builds; treat them as "none running".
        if msg.to_ascii_lowercase().contains("no running") {
            return Ok(Vec::new());
        }
        return Err(msg);
    }

    Ok(parse_distro_list(&stdout))
}

/// Run the /proc reader script inside `distro`, returning its stdout.
fn run_proc_script(distro: &str) -> Result<String, String> {
    let mut cmd = Command::new("wsl.exe");
    cmd.args(["-d", distro, "--", "sh"]);
    let output = run_with_timeout(cmd, Some(PROC_SCRIPT.as_bytes()), WSL_TIMEOUT)?;

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    if !stdout.lines().any(|l| l.starts_with("#\t")) {
        let stderr = decode_wsl_text(&output.stderr);
        return Err(first_meaningful_line(&stderr)
            .unwrap_or_else(|| "distribution did not answer".to_string()));
    }
    Ok(stdout)
}

/// Captured output of a finished (or killed) child process.
struct ChildOutput {
    status: std::process::ExitStatus,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

/// Run `cmd` with optional stdin data, killing it if it exceeds `timeout`.
/// stdout/stderr are drained on helper threads so a chatty child can never
/// dead-lock on a full pipe while we wait.
fn run_with_timeout(mut cmd: Command, stdin_data: Option<&[u8]>, timeout: Duration) -> Result<ChildOutput, String> {
    cmd.creation_flags(CREATE_NO_WINDOW)
        .stdin(if stdin_data.is_some() { Stdio::piped() } else { Stdio::null() })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = cmd
        .spawn()
        .map_err(|e| format!("wsl.exe could not be started ({})", e))?;

    if let (Some(data), Some(mut stdin)) = (stdin_data, child.stdin.take()) {
        // Ignore write errors: if the shell died early the caller sees it in the output.
        let _ = stdin.write_all(data);
        // Dropping stdin closes the pipe so `sh` sees EOF and runs the script.
    }

    let mut stdout = child.stdout.take();
    let mut stderr = child.stderr.take();
    let out_reader = std::thread::spawn(move || {
        let mut buf = Vec::new();
        if let Some(s) = stdout.as_mut() { let _ = std::io::Read::read_to_end(s, &mut buf); }
        buf
    });
    let err_reader = std::thread::spawn(move || {
        let mut buf = Vec::new();
        if let Some(s) = stderr.as_mut() { let _ = std::io::Read::read_to_end(s, &mut buf); }
        buf
    });

    let deadline = Instant::now() + timeout;
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) => {
                if Instant::now() >= deadline {
                    let _ = child.kill();
                    let _ = child.wait();
                    // Make sure the reader threads are released before returning.
                    let _ = out_reader.join();
                    let _ = err_reader.join();
                    return Err(format!("wsl.exe did not respond within {} s", timeout.as_secs()));
                }
                std::thread::sleep(Duration::from_millis(25));
            }
            Err(e) => return Err(format!("failed to wait for wsl.exe ({})", e)),
        }
    };

    let stdout = out_reader.join().unwrap_or_default();
    let stderr = err_reader.join().unwrap_or_default();
    Ok(ChildOutput { status, stdout, stderr })
}

/// Send a Linux signal to a process inside a distribution (used by F9 on the
/// WSL tab). Runs on a helper thread so the TUI never blocks on wsl.exe.
pub fn kill_wsl_process(distro: String, pid: u32, signal: u32) {
    std::thread::spawn(move || {
        let mut cmd = Command::new("wsl.exe");
        cmd.args(["-d", &distro, "--", "kill", &format!("-{}", signal), &pid.to_string()]);
        let _ = run_with_timeout(cmd, None, WSL_TIMEOUT);
    });
}

/// The script executed inside each distribution. Reads /proc with a single
/// awk process (no per-process forks), plus one `ps` call for full command
/// lines, which sidesteps the NUL-separated /proc/<pid>/cmdline format that
/// busybox awk cannot handle.
///
/// Output format (tab separated):
///   #\t<uptime secs>\t<MemTotal kB>\t<CLK_TCK>\t<page size>
///   <pid>\t<ppid>\t<user>\t<state>\t<threads>\t<cpu ticks>\t<rss pages>\t<vsize bytes>\t<starttime ticks>\t<comm>\t<cmdline>
const PROC_SCRIPT: &str = r##"
CLK=$(getconf CLK_TCK 2>/dev/null); [ -n "$CLK" ] || CLK=100
PS=$(getconf PAGESIZE 2>/dev/null); [ -n "$PS" ] || PS=4096
export CLK PS
cd /proc || exit 1
awk 'BEGIN {
  OFS = "\t"
  up = 0; mt = 0
  if ((getline l < "/proc/uptime") > 0) { split(l, a, " "); up = a[1] }
  close("/proc/uptime")
  while ((getline l < "/proc/meminfo") > 0) { if (l ~ /^MemTotal:/) { split(l, a, /[ \t]+/); mt = a[2]; break } }
  close("/proc/meminfo")
  while ((getline l < "/etc/passwd") > 0) { n = split(l, a, ":"); if (n >= 3) uname[a[3]] = a[1] }
  close("/etc/passwd")
  cmd = "ps -eww -o pid=,args= 2>/dev/null || ps -o pid,args 2>/dev/null"
  while ((cmd | getline l) > 0) {
    sub(/^[ \t]+/, "", l)
    sp = index(l, " ")
    if (sp > 0) { p = substr(l, 1, sp - 1); c = substr(l, sp + 1); sub(/^[ \t]+/, "", c); if (p ~ /^[0-9]+$/) args[p] = c }
  }
  close(cmd)
  print "#", up, mt, ENVIRON["CLK"], ENVIRON["PS"]
  for (i = 1; i < ARGC; i++) {
    p = ARGV[i]
    if (p !~ /^[0-9]+$/) continue
    sf = p "/stat"
    if ((getline s < sf) <= 0) { close(sf); continue }
    close(sf)
    lp = index(s, "(")
    rp = 0
    for (k = length(s); k > lp; k--) if (substr(s, k, 1) == ")") { rp = k; break }
    if (lp == 0 || rp == 0) continue
    comm = substr(s, lp + 1, rp - lp - 1)
    n = split(substr(s, rp + 2), f, " ")
    if (n < 22) continue
    uid = ""
    stf = p "/status"
    while ((getline l < stf) > 0) { if (l ~ /^Uid:/) { split(l, a, /[ \t]+/); uid = a[2]; break } }
    close(stf)
    user = (uid in uname) ? uname[uid] : uid
    c = (p in args) ? args[p] : ""
    print p, f[2], user, f[1], f[18], f[12] + f[13], f[22], f[21], f[20], comm, c
  }
  exit
}' [0-9]*
"##;

// ─── Parsing ─────────────────────────────────────────────────────────────────

/// wsl.exe writes UTF-16LE (with or without BOM) to a pipe, but some builds
/// and the Linux side write UTF-8. Detect by looking for NUL bytes.
pub fn decode_wsl_text(bytes: &[u8]) -> String {
    if bytes.len() >= 2 && bytes.iter().skip(1).step_by(2).take(64).any(|&b| b == 0) {
        let start = if bytes.starts_with(&[0xFF, 0xFE]) { 2 } else { 0 };
        let units: Vec<u16> = bytes[start..]
            .chunks_exact(2)
            .map(|c| u16::from_le_bytes([c[0], c[1]]))
            .collect();
        String::from_utf16_lossy(&units)
    } else {
        String::from_utf8_lossy(bytes).into_owned()
    }
}

fn first_meaningful_line(text: &str) -> Option<String> {
    text.lines()
        .map(|l| l.trim().trim_matches('\0'))
        .find(|l| !l.is_empty())
        .map(|l| l.to_string())
}

/// Parse `wsl --list --running --quiet` output into distro names.
pub fn parse_distro_list(text: &str) -> Vec<String> {
    text.lines()
        .map(|l| l.trim().trim_matches('\0').trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string())
        .collect()
}

/// Parse the script output for one distro into process rows, updating the
/// CPU-delta bookkeeping in `prev`.
pub fn parse_script_output(
    distro: &str,
    text: &str,
    prev: &mut HashMap<(String, u32, u64), PrevSample>,
    seen: &mut std::collections::HashSet<(String, u32, u64)>,
) -> Vec<WslProcessInfo> {
    // Skip anything a chatty login shell may have printed before our header.
    let mut lines = text.lines().skip_while(|l| !l.starts_with("#\t"));
    let header = match lines.next() {
        Some(h) => h,
        None => return Vec::new(),
    };
    let hf: Vec<&str> = header.split('\t').collect();
    let uptime: f64 = hf.get(1).and_then(|v| v.trim().parse().ok()).unwrap_or(0.0);
    let mem_total_kb: u64 = hf.get(2).and_then(|v| v.trim().parse().ok()).unwrap_or(0);
    let clk: f64 = hf.get(3).and_then(|v| v.trim().parse().ok()).filter(|&c: &f64| c > 0.0).unwrap_or(100.0);
    let page: u64 = hf.get(4).and_then(|v| v.trim().parse().ok()).filter(|&p| p > 0).unwrap_or(4096);

    let mut out = Vec::new();
    for line in lines {
        let f: Vec<&str> = line.split('\t').collect();
        if f.len() < 10 {
            continue;
        }
        let pid: u32 = match f[0].trim().parse() { Ok(v) => v, Err(_) => continue };
        let ppid: u32 = f[1].trim().parse().unwrap_or(0);
        let user = f[2].trim().to_string();
        let state = f[3].trim().chars().next().unwrap_or('?');
        let threads: u32 = f[4].trim().parse().unwrap_or(1);
        let ticks: u64 = f[5].trim().parse().unwrap_or(0);
        let rss_pages: u64 = f[6].trim().parse().unwrap_or(0);
        let vsize: u64 = f[7].trim().parse().unwrap_or(0);
        let start_ticks: u64 = f[8].trim().parse().unwrap_or(0);
        let comm = f[9].trim().to_string();
        let cmdline = f.get(10).map(|c| c.trim()).unwrap_or("").to_string();

        let key = (distro.to_string(), pid, start_ticks);
        seen.insert(key.clone());

        let cpu_usage = match prev.get(&key) {
            Some(p) if uptime > p.uptime => {
                let dt = uptime - p.uptime;
                let dticks = ticks.saturating_sub(p.ticks) as f64;
                (dticks / clk / dt * 100.0) as f32
            }
            _ => {
                // First sighting: lifetime average, like `ps`.
                let alive = uptime - start_ticks as f64 / clk;
                if alive > 0.5 { (ticks as f64 / clk / alive * 100.0) as f32 } else { 0.0 }
            }
        };
        prev.insert(key, PrevSample { ticks, uptime });

        let resident_mem = rss_pages * page;
        let mem_usage = if mem_total_kb > 0 {
            (resident_mem as f64 / (mem_total_kb as f64 * 1024.0) * 100.0) as f32
        } else {
            0.0
        };

        let command = if cmdline.is_empty() { format!("[{}]", comm) } else { cmdline };

        out.push(WslProcessInfo {
            distro: distro.to_string(),
            pid,
            ppid,
            user,
            state,
            threads,
            cpu_usage,
            mem_usage,
            resident_mem,
            virtual_mem: vsize,
            cpu_time_secs: ticks as f64 / clk,
            name: comm,
            command,
        });
    }
    out
}

/// Format CPU time like htop's TIME+ column (M:SS.cc, H:MM:SS, or Dd HH:MM).
pub fn format_cpu_time(secs: f64) -> String {
    let total_hundredths = (secs * 100.0) as u64;
    let hundredths = total_hundredths % 100;
    let total_seconds = total_hundredths / 100;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    if hours >= 100 {
        format!("{}d{:02}:{:02}", hours / 24, hours % 24, minutes)
    } else if hours > 0 {
        format!("{}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        format!("{}:{:02}.{:02}", minutes, seconds, hundredths)
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_utf16le_distro_list_with_bom() {
        let text = "Ubuntu-22.04\r\ndocker-desktop\r\n";
        let mut bytes = vec![0xFF, 0xFE];
        for u in text.encode_utf16() {
            bytes.extend_from_slice(&u.to_le_bytes());
        }
        let decoded = decode_wsl_text(&bytes);
        assert_eq!(parse_distro_list(&decoded), vec!["Ubuntu-22.04", "docker-desktop"]);
    }

    #[test]
    fn decodes_plain_utf8_too() {
        assert_eq!(parse_distro_list(&decode_wsl_text(b"Debian\n\n")), vec!["Debian"]);
        assert_eq!(parse_distro_list(&decode_wsl_text(b"")), Vec::<String>::new());
    }

    // Captured from an Ubuntu 22.04 WSL2 instance (CLK_TCK=100, 4 KiB pages).
    const SAMPLE_1: &str = "#\t1000.00\t8000000\t100\t4096\n\
1\t0\troot\tS\t1\t150\t2500\t170000000\t0\tsystemd\t/sbin/init\n\
42\t1\tgj\tR\t8\t5000\t250000\t1500000000\t20000\tnode\tnode server.js --port 3000\n\
57\t2\troot\tI\t1\t0\t0\t0\t100\tkworker/u8:1-events_unbound\t\n\
99\t1\tgj\tS\t2\t30\t1000\t8000000\t90000\tbash\t-bash\n";
    const SAMPLE_2: &str = "#\t1002.00\t8000000\t100\t4096\n\
1\t0\troot\tS\t1\t150\t2500\t170000000\t0\tsystemd\t/sbin/init\n\
42\t1\tgj\tR\t8\t5300\t260000\t1500000000\t20000\tnode\tnode server.js --port 3000\n\
99\t1\tgj\tS\t2\t30\t1000\t8000000\t90000\tbash\t-bash\n";

    #[test]
    fn parses_proc_rows_and_computes_cpu_deltas() {
        let mut prev = HashMap::new();
        let mut seen = std::collections::HashSet::new();

        let first = parse_script_output("Ubuntu", SAMPLE_1, &mut prev, &mut seen);
        assert_eq!(first.len(), 4);

        let node = first.iter().find(|p| p.pid == 42).unwrap();
        assert_eq!(node.distro, "Ubuntu");
        assert_eq!(node.user, "gj");
        assert_eq!(node.state, 'R');
        assert_eq!(node.threads, 8);
        assert_eq!(node.name, "node");
        assert_eq!(node.command, "node server.js --port 3000");
        assert_eq!(node.resident_mem, 250000 * 4096);
        // 250000 pages * 4 KiB = 1,024,000,000 B of 8,192,000,000 B = 12.5%
        assert!((node.mem_usage - 12.5).abs() < 0.01, "mem% = {}", node.mem_usage);
        // First sighting: lifetime average = 50 s of CPU over (1000 - 200) s alive = 6.25%
        assert!((node.cpu_usage - 6.25).abs() < 0.01, "cpu% = {}", node.cpu_usage);
        assert_eq!(node.cpu_time_secs, 50.0);

        let kworker = first.iter().find(|p| p.pid == 57).unwrap();
        assert_eq!(kworker.command, "[kworker/u8:1-events_unbound]");
        assert_eq!(kworker.cpu_usage, 0.0);

        // Second sample two seconds later: node used 300 ticks = 3 s CPU in 2 s = 150%
        let mut seen2 = std::collections::HashSet::new();
        let second = parse_script_output("Ubuntu", SAMPLE_2, &mut prev, &mut seen2);
        assert_eq!(second.len(), 3);
        let node2 = second.iter().find(|p| p.pid == 42).unwrap();
        assert!((node2.cpu_usage - 150.0).abs() < 0.01, "cpu% = {}", node2.cpu_usage);
        let bash2 = second.iter().find(|p| p.pid == 99).unwrap();
        assert_eq!(bash2.cpu_usage, 0.0);

        // The kworker vanished, so it must not be in the second "seen" set.
        assert!(!seen2.contains(&("Ubuntu".to_string(), 57, 100)));
    }

    #[test]
    fn recycled_pid_does_not_inherit_old_counters() {
        let mut prev = HashMap::new();
        let mut seen = std::collections::HashSet::new();
        parse_script_output("Ubuntu", SAMPLE_1, &mut prev, &mut seen);
        // Same PID 42 but a different start time and far fewer ticks: a new process.
        let recycled = "#\t1002.00\t8000000\t100\t4096\n\
42\t1\tgj\tR\t1\t10\t100\t100000\t100100\tsleep\tsleep 5\n";
        let rows = parse_script_output("Ubuntu", recycled, &mut prev, &mut seen);
        assert_eq!(rows.len(), 1);
        // Alive for 1 s with 0.1 s of CPU => 10%, not a bogus negative/huge delta.
        assert!((rows[0].cpu_usage - 10.0).abs() < 0.5, "cpu% = {}", rows[0].cpu_usage);
    }

    #[test]
    fn rejects_output_without_header() {
        let mut prev = HashMap::new();
        let mut seen = std::collections::HashSet::new();
        assert!(parse_script_output("X", "sh: awk: not found\n", &mut prev, &mut seen).is_empty());
        assert!(parse_script_output("X", "", &mut prev, &mut seen).is_empty());
    }

    #[test]
    fn skips_shell_noise_before_header() {
        let mut prev = HashMap::new();
        let mut seen = std::collections::HashSet::new();
        let noisy = format!("Welcome to Ubuntu!\nmotd line\n{}", SAMPLE_1);
        assert_eq!(parse_script_output("Ubuntu", &noisy, &mut prev, &mut seen).len(), 4);
    }

    #[test]
    fn formats_cpu_time_like_htop() {
        assert_eq!(format_cpu_time(0.0), "0:00.00");
        assert_eq!(format_cpu_time(50.0), "0:50.00");
        assert_eq!(format_cpu_time(61.25), "1:01.25");
        assert_eq!(format_cpu_time(3661.0), "1:01:01");
    }

    /// Exercises the real wsl.exe path. On a machine without WSL this must
    /// yield a clean Unavailable/NoRunningDistros status rather than a panic.
    #[test]
    fn real_wsl_query_never_panics() {
        let mut prev = HashMap::new();
        let t0 = Instant::now();
        let snap = sample_once(&mut prev);
        // Even a hung wsl.exe must be cut off: one list call plus at most a
        // handful of per-distro calls, each bounded by WSL_TIMEOUT.
        let max = WSL_TIMEOUT * (1 + snap.distros.len() as u32) + Duration::from_secs(2);
        assert!(t0.elapsed() <= max, "sample took {:?}, limit {:?}", t0.elapsed(), max);
        match &snap.status {
            WslStatus::Ok => assert!(!snap.distros.is_empty()),
            WslStatus::NoRunningDistros => assert!(snap.processes.is_empty()),
            WslStatus::Unavailable(msg) => assert!(!msg.is_empty()),
            WslStatus::Pending => panic!("sample_once must resolve the status"),
        }
        eprintln!("wsl status on this machine: {:?}", snap.status);
    }
}
