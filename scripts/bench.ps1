<#
.SYNOPSIS
  Repeatable pstop benchmark: startup, idle and loaded cost, keystroke latency.

.DESCRIPTION
  1. Startup: runs `pstop --bench` -Runs times and records time to first data.
  2. Idle: runs the real TUI in a private psmux server (-L bench-pstop) for
     -Seconds and samples its CPU time and working set.
  3. Load: same while three threads keep three cores busy, so the process
     table has real churn to sort and draw.
  4. Keystroke to screen: sends a key and polls the pane (with colours, since
     the selection bar is only colour) until the screen changes. This includes psmux capture overhead, so compare runs against
     each other, not against an absolute number.

  pstop runs with APPDATA pointed at a temp folder, so your own pstoprc is
  never read or written. Results go to -OutDir as CSV and a JSON summary.

.EXAMPLE
  ./scripts/bench.ps1 -Exe target/release/pstop.exe -Label after
#>
param(
    [string]$Exe = "target/release/pstop.exe",
    [string]$Label = "run",
    [int]$Runs = 15,
    [int]$Seconds = 20,
    [int]$Keys = 15,
    [string]$OutDir = (Join-Path $HOME ("psmux-bench/pstop/" + (Get-Date -Format yyyy-MM-dd)))
)

$ErrorActionPreference = "Stop"
$Exe = (Resolve-Path $Exe).Path
New-Item -ItemType Directory -Force $OutDir | Out-Null
$server = "bench-pstop"
$session = "b"
$fakeAppData = Join-Path ([IO.Path]::GetTempPath()) ("pstop-bench-" + [guid]::NewGuid().ToString("N"))
New-Item -ItemType Directory -Force $fakeAppData | Out-Null
$oldAppData = $env:APPDATA
$rows = New-Object System.Collections.Generic.List[object]

function Add-Row($metric, $iter, $value, $unit) {
    $rows.Add([pscustomobject]@{ label = $Label; metric = $metric; iter = $iter; value = [math]::Round([double]$value, 3); unit = $unit })
}

function Stats($metric) {
    $v = $rows | Where-Object metric -eq $metric | ForEach-Object value | Sort-Object
    if (-not $v) { return $null }
    $v = @($v)
    [pscustomobject]@{
        metric = $metric
        n = $v.Count
        median = $v[[int][math]::Floor(($v.Count - 1) / 2)]
        min = $v[0]
        max = $v[-1]
        mean = [math]::Round(($v | Measure-Object -Average).Average, 3)
    }
}

function Find-Pstop {
    # The pstop process started by our private psmux server.
    Get-CimInstance Win32_Process -Filter "Name='pstop.exe'" |
        Where-Object { $_.ExecutablePath -eq $Exe } |
        Sort-Object CreationDate -Descending | Select-Object -First 1
}

function Sample-Process([int]$procId, [int]$secs, [string]$prefix) {
    $p = Get-Process -Id $procId
    $cpu0 = $p.TotalProcessorTime.TotalMilliseconds
    $t0 = [Diagnostics.Stopwatch]::StartNew()
    for ($i = 1; $i -le $secs; $i++) {
        Start-Sleep -Seconds 1
        $p.Refresh()
        Add-Row "$prefix.working_set_mb" $i ($p.WorkingSet64 / 1MB) "MB"
    }
    $p.Refresh()
    $cpuMs = $p.TotalProcessorTime.TotalMilliseconds - $cpu0
    $pct = 100.0 * $cpuMs / $t0.Elapsed.TotalMilliseconds
    Add-Row "$prefix.cpu_pct_of_one_core" 1 $pct "%"
    Add-Row "$prefix.private_mb" 1 ($p.PrivateMemorySize64 / 1MB) "MB"
}

try {
    $env:APPDATA = $fakeAppData

    # 1. Startup
    for ($i = 1; $i -le $Runs; $i++) {
        $out = & $Exe --bench 2>&1 | Out-String
        if ($out -match "Total to data ready:\s+([\d.]+)ms") { Add-Row "startup.data_ready_ms" $i $Matches[1] "ms" }
        if ($out -match "First refresh\(\):\s+([\d.]+)ms") { Add-Row "startup.first_refresh_ms" $i $Matches[1] "ms" }
    }

    # 2. Idle, in a real terminal
    psmux -L $server kill-server 2>$null | Out-Null
    # The private server starts here, so it and pstop inherit the temp APPDATA.
    psmux -L $server new-session -d -s $session -x 200 -y 50 "$Exe"
    Start-Sleep -Seconds 4
    $proc = Find-Pstop
    if (-not $proc) { throw "pstop did not start in the bench server" }
    Sample-Process $proc.ProcessId $Seconds "idle"

    # 3. Keystroke to screen (Down moves the selection bar)
    for ($i = 1; $i -le $Keys; $i++) {
        $before = psmux -L $server capture-pane -p -e -t $session | Out-String
        $sw = [Diagnostics.Stopwatch]::StartNew()
        psmux -L $server send-keys -t $session $(if ($i % 2) { "Down" } else { "Up" })
        do {
            $now = psmux -L $server capture-pane -p -e -t $session | Out-String
        } while ($now -eq $before -and $sw.ElapsedMilliseconds -lt 3000)
        Add-Row "keystroke_to_screen_ms" $i $sw.Elapsed.TotalMilliseconds "ms"
        Start-Sleep -Milliseconds 200
    }

    # 4. Under load: three busy threads in another process
    $burn = Start-Process pwsh -PassThru -WindowStyle Hidden -ArgumentList @(
        "-NoProfile", "-Command",
        "1..3 | ForEach-Object -Parallel { `$e=(Get-Date).AddSeconds($($Seconds + 10)); while((Get-Date) -lt `$e){} } -ThrottleLimit 3")
    try {
        Start-Sleep -Seconds 3
        Sample-Process $proc.ProcessId $Seconds "load"
    } finally {
        Stop-Process -Id $burn.Id -ErrorAction SilentlyContinue
    }
}
finally {
    $env:APPDATA = $oldAppData
    psmux -L $server kill-server 2>$null | Out-Null
    Remove-Item -Recurse -Force $fakeAppData -ErrorAction SilentlyContinue
}

$ver = (& $Exe --version 2>&1 | Select-Object -First 1)
$csv = Join-Path $OutDir "pstop-$Label.csv"
$rows | Export-Csv -NoTypeInformation -Path $csv
$summary = [pscustomobject]@{
    label = $Label
    version = "$ver"
    exe = $Exe
    machine_logical_cpus = [Environment]::ProcessorCount
    taken = (Get-Date).ToString("o")
    stats = @("startup.data_ready_ms", "startup.first_refresh_ms", "idle.cpu_pct_of_one_core", "idle.working_set_mb",
              "idle.private_mb", "keystroke_to_screen_ms", "load.cpu_pct_of_one_core", "load.working_set_mb",
              "load.private_mb") | ForEach-Object { Stats $_ } | Where-Object { $_ }
}
$summary | ConvertTo-Json -Depth 4 | Set-Content (Join-Path $OutDir "pstop-$Label.json")
$summary.stats | Format-Table -AutoSize
Write-Host "raw: $csv"
