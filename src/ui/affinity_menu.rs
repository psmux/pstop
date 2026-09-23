use ratatui::Frame;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::app::App;

/// Width of one CPU cell in the grid: "[X] CPU 63" + padding.
pub const CELL_WIDTH: u16 = 12;
/// Rows above the CPU grid inside the popup border (title, blank, hint, blank).
const GRID_TOP_ROWS: u16 = 4;
/// Rows below the CPU grid inside the popup border (blank + "Controls:" + 5 control lines).
const GRID_BOTTOM_ROWS: u16 = 7;
/// Minimum popup width so the title and control text never get clipped.
const MIN_WIDTH: u16 = 56;

/// Geometry of the affinity popup, shared by the renderer, the key handler and
/// the mouse handler so that cursor moves and clicks land on exactly the cells
/// that are drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AffinityLayout {
    /// Full popup rectangle (including border)
    pub area: Rect,
    /// Top-left of the CPU grid (inside the border)
    pub grid_x: u16,
    pub grid_y: u16,
    /// CPUs per column; the grid is column-major so Up/Down walk consecutive CPUs
    pub rows: usize,
    pub cols: usize,
}

impl AffinityLayout {
    /// Compute the popup geometry for `cpu_count` CPUs inside a terminal of `term` size.
    /// The grid grows to as many columns as needed so every CPU is visible.
    pub fn compute(cpu_count: usize, term: Rect) -> Self {
        let cpu_count = cpu_count.max(1);

        // Rows available for the grid: terminal height minus border and fixed text.
        let overhead = 2 + GRID_TOP_ROWS + GRID_BOTTOM_ROWS;
        let max_rows = term.height.saturating_sub(overhead).max(1) as usize;

        // Prefer a compact grid: up to 16 rows, then add columns.
        let rows = cpu_count.min(16).min(max_rows).max(1);
        let cols = (cpu_count + rows - 1) / rows;

        let width = (cols as u16 * CELL_WIDTH + 4).max(MIN_WIDTH).min(term.width);
        let height = (rows as u16 + overhead).min(term.height);
        let x = term.x + term.width.saturating_sub(width) / 2;
        let y = term.y + term.height.saturating_sub(height) / 2;
        let area = Rect { x, y, width, height };

        AffinityLayout {
            area,
            grid_x: area.x + 1 + 2, // border + two spaces of indent
            grid_y: area.y + 1 + GRID_TOP_ROWS,
            rows,
            cols,
        }
    }

    /// Map a terminal coordinate to a CPU index, if it falls on a grid cell.
    pub fn cpu_at(&self, cpu_count: usize, x: u16, y: u16) -> Option<usize> {
        if x < self.grid_x || y < self.grid_y {
            return None;
        }
        let col = ((x - self.grid_x) / CELL_WIDTH) as usize;
        let row = (y - self.grid_y) as usize;
        if col >= self.cols || row >= self.rows {
            return None;
        }
        let idx = col * self.rows + row;
        (idx < cpu_count).then_some(idx)
    }
}

/// Draw the CPU Affinity selector (htop 'a')
pub fn draw_affinity_menu(f: &mut Frame, app: &App) {
    let proc = match app.selected_process() {
        Some(p) => p,
        None => return,
    };

    let cpu_count = app.affinity_cpus.len();
    let layout = AffinityLayout::compute(cpu_count, f.area());
    let area = layout.area;
    f.render_widget(Clear, area);

    let selected_count = app.affinity_cpus.iter().filter(|&&on| on).count();

    let mut lines = vec![
        Line::from(Span::styled(
            format!(" CPU Affinity for PID {} - {} ", proc.pid, proc.name),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!(" Allowed CPUs ({} of {} selected):", selected_count, cpu_count),
            Style::default().fg(Color::White),
        )),
        Line::from(""),
    ];

    // Column-major grid: CPU i lives at column i / rows, row i % rows.
    for row in 0..layout.rows {
        let mut spans: Vec<Span> = vec![Span::raw("  ")];
        for col in 0..layout.cols {
            let i = col * layout.rows + row;
            if i >= cpu_count {
                break;
            }
            let on = app.affinity_cpus[i];
            let is_cursor = i == app.affinity_cursor;
            let checkbox = if on { "[X]" } else { "[ ]" };
            let cell = format!("{} CPU {:<3}", checkbox, i);
            let style = if is_cursor {
                Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else if on {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            spans.push(Span::styled(format!("{:<width$}", cell, width = CELL_WIDTH as usize), style));
        }
        lines.push(Line::from(spans));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        " Controls:",
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from("  Arrows/hjkl  Move cursor         Space  Toggle CPU"));
    lines.push(Line::from("  0-9          Toggle CPU 0-9      Click  Toggle CPU"));
    lines.push(Line::from("  a            Toggle all          i      Invert"));
    lines.push(Line::from("  Home / End   First / last CPU"));
    lines.push(Line::from("  Enter        Apply and close     Esc    Cancel"));

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" CPU Affinity ")
                .title_alignment(Alignment::Center)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .style(Style::default().fg(Color::White).bg(Color::Black));

    f.render_widget(paragraph, area);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_shows_every_cpu_on_a_24_thread_machine() {
        let term = Rect { x: 0, y: 0, width: 120, height: 40 };
        let l = AffinityLayout::compute(24, term);
        assert_eq!(l.rows, 16);
        assert_eq!(l.cols, 2);
        // Every CPU index maps to a distinct cell and back.
        for i in 0..24 {
            let col = (i / l.rows) as u16;
            let row = (i % l.rows) as u16;
            let x = l.grid_x + col * CELL_WIDTH + 3;
            let y = l.grid_y + row;
            assert_eq!(l.cpu_at(24, x, y), Some(i), "cpu {} click mapping", i);
        }
        // A click in the unused part of the last column is ignored.
        assert_eq!(l.cpu_at(24, l.grid_x + CELL_WIDTH, l.grid_y + 10), None);
        // The popup fits inside the terminal.
        assert!(l.area.x + l.area.width <= term.width);
        assert!(l.area.y + l.area.height <= term.height);
    }

    #[test]
    fn renders_all_24_cpus_in_the_popup() {
        use ratatui::backend::TestBackend;
        use ratatui::Terminal;
        use crate::system::process::ProcessInfo;

        let mut app = App::new();
        let p = ProcessInfo {
            pid: 4242,
            ppid: 1,
            name: "game.exe".to_string(),
            command: "game.exe".to_string(),
            user: "gj".to_string(),
            status: crate::system::process::ProcessStatus::Running,
            priority: 8,
            nice: 0,
            virtual_mem: 0,
            resident_mem: 0,
            shared_mem: 0,
            cpu_usage: 0.0,
            mem_usage: 0.0,
            run_time: 0,
            cpu_time_100ns: 0,
            threads: 1,
            io_read_rate: 0.0,
            io_write_rate: 0.0,
            depth: 0,
            is_last_child: false,
        };
        app.filtered_processes = vec![p];
        app.affinity_cpus = (0..24).map(|i| i % 3 != 0).collect();
        app.affinity_cursor = 17;

        let backend = TestBackend::new(100, 34);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|f| draw_affinity_menu(f, &app)).unwrap();

        let buf = terminal.backend().buffer().clone();
        let mut text = String::new();
        for y in 0..buf.area.height {
            for x in 0..buf.area.width {
                text.push_str(buf[(x, y)].symbol());
            }
            text.push('\n');
        }
        println!("{}", text);
        for i in 0..24 {
            assert!(text.contains(&format!("CPU {}", i)), "CPU {} missing from popup", i);
        }
        assert!(text.contains("[ ] CPU 0 "));
        assert!(text.contains("[X] CPU 1 "));
        assert!(text.contains("[X] CPU 23"));
        assert!(text.contains("16 of 24 selected"));
    }

    #[test]
    fn grid_adds_columns_on_short_terminals() {
        let term = Rect { x: 0, y: 0, width: 100, height: 24 };
        let l = AffinityLayout::compute(64, term);
        assert!(l.rows * l.cols >= 64);
        assert!(l.area.height <= term.height);
        let last_col_x = l.grid_x + (l.cols as u16 - 1) * CELL_WIDTH;
        assert_eq!(l.cpu_at(64, last_col_x, l.grid_y), Some((l.cols - 1) * l.rows));
    }
}
