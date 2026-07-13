//! Build script for pstop.
//!
//! Captures the exact git commit the binary was built from and exposes it to
//! the crate as compile-time environment variables:
//!
//!   * `PSTOP_GIT_HASH`      — short commit hash (e.g. `56b742b`), or `unknown`
//!   * `PSTOP_GIT_HASH_FULL` — full 40-char commit hash, or `unknown`
//!   * `PSTOP_GIT_DIRTY`     — `true` if the working tree had uncommitted
//!                             changes at build time, otherwise `false`
//!   * `PSTOP_GIT_DATE`      — commit date (YYYY-MM-DD), or `unknown`
//!
//! Git may be unavailable (for example when the crate is built from a
//! crates.io tarball rather than a git checkout). In that case every value
//! falls back gracefully to `unknown` / `false` so the build always succeeds.

use std::process::Command;

fn main() {
    // Rebuild whenever HEAD moves (new commit, checkout, etc.) so the embedded
    // hash never goes stale. These files may not exist in a tarball build; the
    // rerun hints are harmless if absent.
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs");

    let short = git(&["rev-parse", "--short", "HEAD"]).unwrap_or_else(|| "unknown".to_string());
    let full = git(&["rev-parse", "HEAD"]).unwrap_or_else(|| "unknown".to_string());
    let date =
        git(&["show", "-s", "--format=%cd", "--date=short", "HEAD"]).unwrap_or_else(|| "unknown".to_string());

    // "Dirty" means tracked, version-controlled source differs from HEAD, i.e.
    // the binary can no longer be reproduced from the named commit alone.
    // `--untracked-files=no` deliberately ignores untracked files (build tool
    // state, editor scratch, etc.) so they never produce a false "dirty" flag.
    let dirty = git(&["status", "--porcelain", "--untracked-files=no"])
        .map(|s| !s.is_empty())
        .unwrap_or(false);

    println!("cargo:rustc-env=PSTOP_GIT_HASH={short}");
    println!("cargo:rustc-env=PSTOP_GIT_HASH_FULL={full}");
    println!("cargo:rustc-env=PSTOP_GIT_DIRTY={dirty}");
    println!("cargo:rustc-env=PSTOP_GIT_DATE={date}");
}

/// Run `git <args>` and return trimmed stdout on success, or `None` if git is
/// missing or the command failed.
fn git(args: &[&str]) -> Option<String> {
    let output = Command::new("git").args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}
