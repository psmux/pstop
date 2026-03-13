pub mod header;
pub mod process_table;
pub mod footer;
pub mod help;
pub mod sort_menu;
pub mod kill_menu;
pub mod user_menu;
pub mod affinity_menu;
pub mod environment_view;
pub mod setup_menu;
pub mod handles_view;
pub mod tab_bar;

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};

use crate::app::{App, AppMode};

/// Minimum width (chars) for a single CPU bar column to remain readable.
/// At 8 chars: " 96[|99%]" — cramped but functional, matching htop on narrow terminals.
const MIN_CPU_COL_WIDTH: u16 = 8;

/// Preferred minimum width for comfortable display.
const PREFERRED_CPU_COL_WIDTH: u16 = 15;

/// Calculate the optimal number of CPU columns based on core count and terminal size.
/// Returns an even number (2, 4, 8, 16, 32). Always even for left/right panel symmetry.
///
/// htop meter equivalents:
///   AllCPUs  = 1 sub-col per panel = 2 total   (default for ≤16 cores)
///   AllCPUs2 = 2 sub-cols per panel = 4 total   (default for ≤32 cores)
///   AllCPUs4 = 4 sub-cols per panel = 8 total   (default for ≤64 cores)
///   AllCPUs8 = 8 sub-cols per panel = 16 total  (default for >64 cores)
///
/// The auto-selection picks the smallest column count that fits the available
/// header height, then falls back to the largest even count that fits the width.
pub fn cpu_column_count(core_count: usize, term_height: u16, term_width: u16) -> usize {
    if core_count <= 1 {
        return 2;
    }

    // Max header height: up to 40% of terminal, but at least 6 rows.
    // Never let header exceed terminal - 7 (tab bar + footer + min 5 process rows).
    let max_header = ((term_height as usize) * 2 / 5)
        .max(6)
        .min((term_height as usize).saturating_sub(7));
    let max_cpu_rows = max_header.saturating_sub(3); // 3 rows for info meters (Mem/Swap/Net or Tasks/Load/Uptime)
    if max_cpu_rows == 0 {
        return 2;
    }

    // First pass: try with preferred width (comfortable display)
    let preferred_cols = (term_width / PREFERRED_CPU_COL_WIDTH) as usize;
    let preferred_cols = preferred_cols.max(2);

    for &cols in &[2, 4, 8, 16, 32] {
        if cols > preferred_cols {
            break;
        }
        let rows_needed = (core_count + cols - 1) / cols;
        if rows_needed <= max_cpu_rows {
            return cols;
        }
    }

    // Second pass: try with minimum width (tight display for high core counts)
    let tight_cols = (term_width / MIN_CPU_COL_WIDTH) as usize;
    let tight_cols = tight_cols.max(2);

    for &cols in &[2, 4, 8, 16, 32] {
        if cols > tight_cols {
            break;
        }
        let rows_needed = (core_count + cols - 1) / cols;
        if rows_needed <= max_cpu_rows {
            return cols;
        }
    }

    // Fallback: use the largest even column count that fits the width.
    // This minimizes rows even if header still exceeds preferred max height.
    let max_even = ((tight_cols / 2) * 2).max(2).min(32);
    max_even
}

/// Check if a meter name represents CPU bars
fn is_cpu_meter_name(name: &str) -> bool {
    matches!(name, "AllCPUs" | "AllCPUs2" | "AllCPUs4" | "AllCPUs8") || name.starts_with("CPUs")
}

/// Get forced sub-column count from meter variant name
fn meter_subcols(name: &str) -> Option<usize> {
    match name {
        "AllCPUs2" => Some(2),
        "AllCPUs4" => Some(4),
        "AllCPUs8" => Some(8),
        _ => None,
    }
}

/// Calculate the header height based on number of CPU cores and terminal size.
/// htop-style: each panel flows independently, so height = max(left, right).
pub fn header_height(app: &App, term_height: u16, term_width: u16) -> u16 {
    if app.compact_mode {
        return 2; // 1 aggregate CPU bar + 1 Mem bar
    }
    let cores = app.cpu_info.cores.len();
    if cores == 0 {
        return 5; // fallback: just info rows
    }

    // Count non-CPU info meters per panel
    let left_has_cpus = app.left_meters.iter().any(|m| is_cpu_meter_name(m));
    let right_has_cpus = app.right_meters.iter().any(|m| is_cpu_meter_name(m));
    let left_info_count = app.left_meters.iter().filter(|m| !is_cpu_meter_name(m)).count();
    let right_info_count = app.right_meters.iter().filter(|m| !is_cpu_meter_name(m)).count();

    // Auto-detect column count
    let auto_cpu_cols = cpu_column_count(cores, term_height, term_width);
    let auto_sub_cols = (auto_cpu_cols / 2).max(1);

    // Per-panel sub-column counts (respect explicit AllCPUs2/4/8 variants)
    let left_sub_cols = app.left_meters.iter()
        .find(|m| is_cpu_meter_name(m))
        .and_then(|m| meter_subcols(m))
        .unwrap_or(auto_sub_cols);
    let right_sub_cols = app.right_meters.iter()
        .find(|m| is_cpu_meter_name(m))
        .and_then(|m| meter_subcols(m))
        .unwrap_or(auto_sub_cols);

    let (left_cores, right_cores) = if left_has_cpus && right_has_cpus {
        let half = (cores + 1) / 2;
        (half, cores - half)
    } else if left_has_cpus {
        (cores, 0)
    } else if right_has_cpus {
        (0, cores)
    } else {
        (0, 0)
    };

    let left_cpu_rows = if left_cores > 0 {
        (left_cores + left_sub_cols - 1) / left_sub_cols
    } else {
        0
    };
    let right_cpu_rows = if right_cores > 0 {
        (right_cores + right_sub_cols - 1) / right_sub_cols
    } else {
        0
    };

    let left_total = left_cpu_rows + left_info_count;
    let right_total = right_cpu_rows + right_info_count;
    let pad: usize = if app.header_margin { 2 } else { 0 };
    (left_total.max(right_total).max(1) + pad) as u16
}

/// Render the complete UI
pub fn draw(f: &mut Frame, app: &App) {
    let size = f.area();
    let h_height = header_height(app, size.height, size.width);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(h_height),   // header (CPU + mem + info)
            Constraint::Length(1),          // tab bar (Main | I/O)
            Constraint::Min(5),             // process table
            Constraint::Length(1),          // footer (F-key bar)
        ])
        .split(size);

    header::draw_header(f, app, chunks[0], size.height, size.width);
    tab_bar::draw_tab_bar(f, app, chunks[1]);
    process_table::draw_process_table(f, app, chunks[2]);
    footer::draw_footer(f, app, chunks[3]);

    // Overlay popups
    match app.mode {
        AppMode::Help => help::draw_help(f),
        AppMode::Setup => setup_menu::draw_setup_menu(f, app),
        AppMode::SortSelect => sort_menu::draw_sort_menu(f, app),
        AppMode::Kill => kill_menu::draw_kill_menu(f, app),
        AppMode::UserFilter => user_menu::draw_user_menu(f, app),
        AppMode::Affinity => affinity_menu::draw_affinity_menu(f, app),
        AppMode::Environment => environment_view::draw_environment_view(f, app),
        AppMode::Handles => handles_view::draw_handles_view(f, app),
        _ => {}
    }
}
