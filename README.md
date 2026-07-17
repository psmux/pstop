<p align="center">
<pre>
██████╗ ███████╗████████╗ ██████╗ ██████╗
██╔══██╗██╔════╝╚══██╔══╝██╔═══██╗██╔══██╗
██████╔╝███████╗   ██║   ██║   ██║██████╔╝
██╔═══╝ ╚════██║   ██║   ██║   ██║██╔═══╝
██║     ███████║   ██║   ╚██████╔╝██║
╚═╝     ╚══════╝   ╚═╝    ╚═════╝ ╚═╝
</pre>
</p>

<h3 align="center">The <code>htop</code> alternative for Windows PowerShell.</h3>

<p align="center">
  <strong>Beautiful, fast, real-time system monitor for Windows. Built in Rust.</strong>
</p>

<p align="center">
  <a href="#installation">Install</a> •
  <a href="#features">Features</a> •
  <a href="#keybindings">Keys</a> •
  <a href="#color-schemes">Themes</a> •
  <a href="#configuration">Config</a> •
  <a href="#license">License</a>
</p>

<p align="center">
  <a href="https://crates.io/crates/pstop"><img src="https://img.shields.io/crates/v/pstop?style=flat-square&color=fc8d62&logo=rust" alt="crates.io"/></a>
  <a href="https://community.chocolatey.org/packages/pstop"><img src="https://img.shields.io/chocolatey/v/pstop?style=flat-square&color=7B3F00&logo=chocolatey" alt="Chocolatey"/></a>
  <a href="https://github.com/microsoft/winget-pkgs/tree/master/manifests/m/marlocarlo/pstop"><img src="https://img.shields.io/badge/winget-marlocarlo.pstop-0078D4?style=flat-square&logo=windows" alt="WinGet"/></a>
  <img src="https://img.shields.io/badge/platform-windows-blue?style=flat-square&logo=windows" alt="Windows"/>
  <img src="https://img.shields.io/badge/language-rust-orange?style=flat-square&logo=rust" alt="Rust"/>
  <img src="https://img.shields.io/badge/license-MIT-green?style=flat-square" alt="MIT License"/>
  <img src="https://img.shields.io/badge/terminal-powershell-5391FE?style=flat-square&logo=powershell&logoColor=white" alt="PowerShell"/>
</p>

---

<p align="center">
  <img src="pstop-demo.gif" alt="pstop demo - htop for Windows PowerShell" width="900"/>
</p>

---

## Why pstop?

If you've ever missed `htop` on Windows, your search is over. **pstop** brings the full htop experience to Windows PowerShell. No WSL, no Cygwin, no compromises.

| | **pstop** | Task Manager | `Get-Process` |
|---|:---:|:---:|:---:|
| Real-time CPU per-core bars | ✅ | ❌ | ❌ |
| Memory / Swap / Network bars | ✅ | Partial | ❌ |
| GPU utilization & VRAM bars | ✅ | Basic | ❌ |
| Tree view (process hierarchy) | ✅ | ❌ | ❌ |
| Search & filter processes | ✅ | Basic | ❌ |
| Kill / change priority | ✅ | ✅ | Manual |
| Mouse support | ✅ | ✅ | ❌ |
| 23 color schemes | ✅ | ❌ | ❌ |
| Keyboard-driven | ✅ | ❌ | ❌ |
| Runs in terminal | ✅ | ❌ | ✅ |
| ~1 MB binary, zero dependencies | ✅ | N/A | N/A |

---

## Installation

### WinGet (Recommended)

```powershell
winget install marlocarlo.pstop
```

### Chocolatey

```powershell
choco install pstop
```

### Cargo (crates.io)

```powershell
cargo install pstop
```

> **Don't have Rust/Cargo?** Install it in seconds: <https://rustup.rs>

### From GitHub Releases

Download the latest `.zip` from [GitHub Releases](https://github.com/psmux/pstop/releases), extract, and add to your `PATH`.

### From Source

```powershell
cargo install --git https://github.com/psmux/pstop
```

### Build Locally

```powershell
git clone https://github.com/psmux/pstop.git
cd pstop
cargo build --release
# Binary at: target/release/pstop.exe + target/release/htop.exe
```

All methods install **both** `pstop` and `htop` commands. Yes, you can just type `htop` on Windows.

### Add htop Alias (Optional)

If you only installed `pstop` and want the `htop` alias in your PowerShell profile:

```powershell
pstop --install-alias
```

This adds `Set-Alias htop pstop` to your `$PROFILE` automatically.

---

## Features

### 🖥️ Per-Core CPU Monitoring
Real-time CPU usage bars for every logical core, color-coded by usage type (user / system / virtual), exactly like htop. CPU columns auto-adjust based on core count (2/4/8/16 columns) and terminal size — just like htop's `calcColumnWidthCount`.

### 📊 Memory, Swap, Network & GPU Bars
- **Mem** bar: shows used (green), buffers (blue), cached (yellow)
- **Swap** bar: swap usage with color threshold
- **Net** bar: live RX/TX throughput in the header
- **GPU** bar: overall GPU utilization percentage (shown on GPU tab)
- **VMem** bar: dedicated video memory usage (shown on GPU tab)

### 🌳 Tree View
Press `F5` or `t` to toggle process tree view — see parent-child relationships with `├─` / `└─` tree connectors, collapsible nodes with `+`/`-`.

### 🔍 Search & Filter
- **F3** - Incremental search: jumps to matching process
- **F4** - Filter: hides all non-matching processes in real-time

### 📋 Four Tab Views
- **Main** - Full process table (PID, USER, CPU%, MEM%, TIME+, Command...)
- **I/O** - Disk read/write rates per process
- **Net** - Per-process network bandwidth (live download/upload rates with auto-scaling B/s, KB/s, MB/s, GB/s) plus active connection counts. No admin required.
- **GPU** - Per-process GPU engine utilization and dedicated/shared video memory usage via PDH performance counters

### ⚙️ F2 Setup Menu (Full htop Parity)
Press `F2` to open the setup menu with 4 categories:
- **Meters** - Configure header layout (CPU, Memory, Swap, Network, Tasks, Load, Uptime)
- **Display Options** - 15 toggleable settings (tree view, highlight basename, shadow other users, show threads, detailed CPU time, vim keys, and more)
- **Colors** - Choose from 23 built-in color schemes with **live preview**
- **Columns** - Add/remove/reorder visible columns

### 🎨 23 Color Schemes
Switch instantly in F2 > Colors. Choose from built-in htop schemes plus all 16 Windows Terminal color schemes:

**Original htop Schemes:**
1. **Default** - Classic htop green/cyan on black
2. **Monochrome** - Pure white on black
3. **Black Night** - Muted tones for dark terminals
4. **Light Terminal** - Optimized for light backgrounds
5. **Midnight Commander** - Blue background, MC-inspired
6. **Black on White** - Clean light theme
7. **Dark Vivid** - High-contrast neon colors

**Windows Terminal Built-in Schemes:**
8. **CGA** - Classic color graphics adapter palette
9. **Campbell** - Windows Terminal default scheme
10. **Campbell Powershell** - Campbell on the classic PowerShell blue
11. **Dark+** - VS Code classic dark theme
12. **Dimidium** - Warm muted tones
13. **IBM 5153** - Retro IBM terminal colors
14. **One Half Dark** - Atom One Half Dark theme
15. **One Half Light** - Atom One Half Light theme
16. **Ottosson** - Perceptually balanced palette by Björn Ottosson
17. **Solarized Dark** - Solarized dark mode
18. **Solarized Light** - Solarized light mode
19. **Tango Dark** - Tango desktop environment dark
20. **Tango Light** - Tango desktop environment light
21. **Vintage** - Classic terminal colors
22. **VSCode Dark Modern** - VS Code modern dark theme
23. **VSCode Light Modern** - VS Code modern light theme

### 🖱️ Full Mouse Support
- Click anywhere in the process table to select
- Click column headers to sort
- Click F-key bar buttons
- Click tabs to switch views
- Scroll wheel for navigation

### ⌨️ Keyboard Shortcuts
Familiar htop keybindings — zero learning curve if you know htop.

### 💾 Persistent Configuration
All settings auto-save to `%APPDATA%/pstop/pstoprc` and restore on next launch. Your color scheme, display options, column choices, sort preference... everything persists.

### ⚡ Performance
- ~1 MB single binary (release build with LTO + strip)
- 50ms event polling for instant keyboard response
- Configurable refresh rate (200ms–10s)
- Native Win32 API calls for I/O counters, process priority, CPU affinity
- Zero runtime dependencies

---

## Keybindings

| Key | Action |
|-----|--------|
| `F1` / `?` | Help screen |
| `F2` | Setup menu (meters, display, colors, columns) |
| `F3` / `/` | Search processes |
| `F4` / `\` | Filter processes |
| `F5` / `t` | Toggle tree view |
| `F6` / `>` | Sort by column |
| `F7` / `F8` | Decrease / Increase process priority (nice) |
| `F9` / `k` | Kill process |
| `F10` / `q` | Quit |
| `Tab` | Switch between Main / I/O / Net / GPU views |
| `Space` | Tag process |
| `c` | Tag process and children |
| `U` | Untag all |
| `u` | Filter by user |
| `p` | Toggle full command path / process name |
| `H` | Toggle show threads |
| `K` | Toggle hide kernel threads |
| `+` / `-` | Expand / collapse tree node |
| `e` | Show process environment |
| `l` | List open handles (lsof equivalent) |
| `a` | Set CPU affinity |
| `I` | Invert sort order |
| Arrow keys | Navigate |
| `PgUp` / `PgDn` | Page through process list |
| `Home` / `End` | Jump to first / last process |

### Vim Mode (opt-in)

Enable via `F2` > Display Options > **Vim-style keys**, or set `vim_keys=1` in your config file. Off by default.

| Key | Vim Mode Action | Replaces |
|-----|----------------|----------|
| `j` | Move down | *(new — default mode has no bare `j`)* |
| `k` | Move up | `k` = kill in default mode |
| `g` | Jump to first process | `Home` |
| `G` | Jump to last process | `End` |
| `Ctrl+d` | Half page down | *(new)* |
| `Ctrl+u` | Half page up | *(new)* |
| `x` | Kill process | `k` / `F9` |
| `/` | Search | *(unchanged, works in both modes)* |
| `?` | Help | *(unchanged, works in both modes)* |

**What changes in vim mode:**
- `k` becomes **move up** instead of kill — use `x` or `F9` to kill
- `h` no longer opens help — use `?` or `F1` instead
- `j`/`k` work as bare keys (no `Alt` modifier needed)
- All other keys (`F1`–`F10`, `Space`, `u`, `t`, `e`, `l`, `a`, etc.) remain unchanged

**What stays the same:**
- Arrow keys, PgUp/PgDn, Home/End still work
- All F-key shortcuts (`F1`–`F10`) still work
- `/` for search, `\` for filter
- `q` to quit, `Ctrl+C` to quit
- All sorting keys (`P`, `M`, `T`, `N`, `I`, `<`, `>`)
- Tree view (`t`/`F5`), tags (`Space`/`c`/`U`), user filter (`u`)

---

## Color Schemes

All 23 schemes affect every UI element: header bars, process table, footer, tabs, popups. Each scheme provides a cohesive color palette optimized for readability and visual hierarchy.

The original 7 htop schemes remain the core collection, with 16 Windows Terminal schemes added for users who want consistency across terminal applications.

**Quick Reference:**
- Need dark mode? Try **Default**, **Dark+**, **One Half Dark**, or **Solarized Dark**
- Need light mode? Try **Light Terminal**, **One Half Light**, **Solarized Light**, or **VSCode Light Modern**
- Retro vibes? Try **CGA**, **IBM 5153**, **Campbell**, or **Vintage**
- High contrast? Try **Dark Vivid**, **Monochrome**, or **VSCode Dark Modern**

Change schemes live: `F2` > Colors > select > `Enter`. Preview updates in real-time.

---

## Configuration

Settings are saved automatically to:

```
%APPDATA%\pstop\pstoprc
```

Format: simple `key=value` (htoprc-style). Persisted settings include:
- Color scheme
- All 15 display options (including vim keys mode)
- Visible columns
- Sort field & direction
- Update interval
- Tree view state

To enable vim keys from the config file directly:
```
vim_keys=1
```

---

## System Requirements

- **OS**: Windows 10 / 11 (x86_64)
- **Terminal**: Windows Terminal, PowerShell, cmd.exe, or any terminal with ANSI support
- **Build**: Rust 1.70+ (for building from source)

---

## Roadmap

- [x] Publish to crates.io (`cargo install pstop`)
- [x] Pre-built binaries via GitHub Releases
- [x] WinGet (`winget install marlocarlo.pstop`)
- [x] Chocolatey (`choco install pstop`)
- [x] GPU monitoring (per-process GPU engine usage + VRAM, header GPU/VMem bars)
- [x] Network per-process tracking (live bandwidth, no admin required)
- [x] Auto-adjusting CPU column layout (2/4/8/16 columns based on core count)
- [x] Independent htop-style header panel flow (no forced alignment)
- [x] Vim-style keybindings (opt-in `j`/`k`/`g`/`G`/`Ctrl-u`/`Ctrl-d`)
- [ ] Scoop bucket
- [ ] Custom meter plugins

---

## Contributing

Contributions welcome! This is a Rust project using:
- **ratatui** 0.30 - TUI framework
- **crossterm** 0.29 - Terminal backend
- **sysinfo** 0.38 - System information
- **windows** 0.62 - Native Win32 APIs

```powershell
git clone https://github.com/psmux/pstop.git
cd pstop
cargo run
```

---

## License

[MIT](LICENSE) - use it, fork it, ship it.

---

<p align="center">
  <strong>Stop opening Task Manager. Type <code>pstop</code> or its aliases.</strong>
</p>
