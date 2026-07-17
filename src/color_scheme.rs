use ratatui::style::{Color, Modifier, Style};

/// All htop color scheme variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSchemeId {
    Default = 0,
    Monochrome = 1,
    BlackNight = 2,
    LightTerminal = 3,
    MidnightCommander = 4,
    BlackOnWhite = 5,
    DarkVivid = 6,
    Cga = 7,
    Campbell = 8,
    CampbellPowershell = 9,
    DarkPlus = 10,
    Dimidium = 11,
    Ibm5153 = 12,
    OneHalfDark = 13,
    OneHalfLight = 14,
    Ottosson = 15,
    SolarizedDark = 16,
    SolarizedLight = 17,
    TangoDark = 18,
    TangoLight = 19,
    Vintage = 20,
    VscodeDarkModern = 21,
    VscodeLightModern = 22,
}

impl ColorSchemeId {
    pub fn all() -> &'static [ColorSchemeId] {
        &[
            ColorSchemeId::Default,
            ColorSchemeId::Monochrome,
            ColorSchemeId::BlackNight,
            ColorSchemeId::LightTerminal,
            ColorSchemeId::MidnightCommander,
            ColorSchemeId::BlackOnWhite,
            ColorSchemeId::DarkVivid,
            ColorSchemeId::Cga,
            ColorSchemeId::Campbell,
            ColorSchemeId::CampbellPowershell,
            ColorSchemeId::DarkPlus,
            ColorSchemeId::Dimidium,
            ColorSchemeId::Ibm5153,
            ColorSchemeId::OneHalfDark,
            ColorSchemeId::OneHalfLight,
            ColorSchemeId::Ottosson,
            ColorSchemeId::SolarizedDark,
            ColorSchemeId::SolarizedLight,
            ColorSchemeId::TangoDark,
            ColorSchemeId::TangoLight,
            ColorSchemeId::Vintage,
            ColorSchemeId::VscodeDarkModern,
            ColorSchemeId::VscodeLightModern,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            ColorSchemeId::Default => "Default",
            ColorSchemeId::Monochrome => "Monochrome",
            ColorSchemeId::BlackNight => "Black Night",
            ColorSchemeId::LightTerminal => "Light Terminal",
            ColorSchemeId::MidnightCommander => "MC",
            ColorSchemeId::BlackOnWhite => "Black on White",
            ColorSchemeId::DarkVivid => "Dark Vivid",
            ColorSchemeId::Cga => "CGA",
            ColorSchemeId::Campbell => "Campbell",
            ColorSchemeId::CampbellPowershell => "Campbell PowerShell",
            ColorSchemeId::DarkPlus => "Dark+",
            ColorSchemeId::Dimidium => "Dimidium",
            ColorSchemeId::Ibm5153 => "IBM 5153",
            ColorSchemeId::OneHalfDark => "One Half Dark",
            ColorSchemeId::OneHalfLight => "One Half Light",
            ColorSchemeId::Ottosson => "Ottosson",
            ColorSchemeId::SolarizedDark => "Solarized Dark",
            ColorSchemeId::SolarizedLight => "Solarized Light",
            ColorSchemeId::TangoDark => "Tango Dark",
            ColorSchemeId::TangoLight => "Tango Light",
            ColorSchemeId::Vintage => "Vintage",
            ColorSchemeId::VscodeDarkModern => "VSCode Dark Modern",
            ColorSchemeId::VscodeLightModern => "VSCode Light Modern",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ColorSchemeId::Default => "htop classic dark theme",
            ColorSchemeId::Monochrome => "No colors, monochrome",
            ColorSchemeId::BlackNight => "Dark theme for dark terminals",
            ColorSchemeId::LightTerminal => "For light terminal backgrounds",
            ColorSchemeId::MidnightCommander => "Midnight Commander style",
            ColorSchemeId::BlackOnWhite => "Black text on white background",
            ColorSchemeId::DarkVivid => "Vivid dark colors with contrast",
            ColorSchemeId::Cga => "Windows Terminal CGA retro palette",
            ColorSchemeId::Campbell => "Windows Terminal default dark scheme",
            ColorSchemeId::CampbellPowershell => "Campbell scheme with PowerShell blue background",
            ColorSchemeId::DarkPlus => "VS Code Dark+ inspired theme",
            ColorSchemeId::Dimidium => "Windows Terminal Dimidium dark theme",
            ColorSchemeId::Ibm5153 => "Retro IBM 5153 CGA monitor palette",
            ColorSchemeId::OneHalfDark => "Atom One Half Dark theme",
            ColorSchemeId::OneHalfLight => "Atom One Half Light theme",
            ColorSchemeId::Ottosson => "Windows Terminal Ottosson dark theme",
            ColorSchemeId::SolarizedDark => "Solarized dark theme",
            ColorSchemeId::SolarizedLight => "Solarized light theme",
            ColorSchemeId::TangoDark => "Tango dark palette",
            ColorSchemeId::TangoLight => "Tango light palette",
            ColorSchemeId::Vintage => "Classic vintage terminal colors",
            ColorSchemeId::VscodeDarkModern => "VS Code Dark Modern theme",
            ColorSchemeId::VscodeLightModern => "VS Code Light Modern theme",
        }
    }

    pub fn from_index(idx: usize) -> Self {
        match idx {
            0 => ColorSchemeId::Default,
            1 => ColorSchemeId::Monochrome,
            2 => ColorSchemeId::BlackNight,
            3 => ColorSchemeId::LightTerminal,
            4 => ColorSchemeId::MidnightCommander,
            5 => ColorSchemeId::BlackOnWhite,
            6 => ColorSchemeId::DarkVivid,
            7 => ColorSchemeId::Cga,
            8 => ColorSchemeId::Campbell,
            9 => ColorSchemeId::CampbellPowershell,
            10 => ColorSchemeId::DarkPlus,
            11 => ColorSchemeId::Dimidium,
            12 => ColorSchemeId::Ibm5153,
            13 => ColorSchemeId::OneHalfDark,
            14 => ColorSchemeId::OneHalfLight,
            15 => ColorSchemeId::Ottosson,
            16 => ColorSchemeId::SolarizedDark,
            17 => ColorSchemeId::SolarizedLight,
            18 => ColorSchemeId::TangoDark,
            19 => ColorSchemeId::TangoLight,
            20 => ColorSchemeId::Vintage,
            21 => ColorSchemeId::VscodeDarkModern,
            22 => ColorSchemeId::VscodeLightModern,
            _ => ColorSchemeId::Default,
        }
    }
}

/// All configurable color slots used across the app
#[derive(Debug, Clone)]
pub struct ColorScheme {
    // Background
    pub bg: Color,

    // Header bars
    pub cpu_bar_normal: Color,     // User/normal processes
    pub cpu_bar_system: Color,     // Kernel/system
    pub cpu_bar_low: Color,        // Low priority (nice > 0)
    pub cpu_bar_virt: Color,       // Virtual/steal/guest
    pub cpu_bar_iowait: Color,     // IO wait (detailed mode)
    pub cpu_bar_irq: Color,        // Hardware interrupt / IRQ (detailed mode)
    pub cpu_bar_softirq: Color,    // DPC / soft IRQ (detailed mode)
    pub cpu_label: Color,          // CPU number label
    pub cpu_bar_bg: Color,         // Bar background char color

    pub mem_bar_used: Color,       // Used memory
    pub mem_bar_buffers: Color,    // Buffers
    pub mem_bar_cache: Color,      // Cache

    pub swap_bar: Color,           // Swap usage

    // Header info text
    pub tasks_text: Color,
    pub load_text: Color,
    pub uptime_text: Color,
    pub info_label: Color,         // "Tasks:", "Load average:" labels
    pub info_value: Color,         // Values/numbers

    // Process table header
    pub table_header_bg: Color,
    pub table_header_fg: Color,
    pub table_header_sort_bg: Color,
    pub table_header_sort_fg: Color,

    // Process table rows
    pub process_fg: Color,          // Default process text
    pub process_bg: Color,          // Default row background
    pub process_selected_bg: Color, // Selected row background
    pub process_selected_fg: Color, // Selected row fg
    pub process_shadow: Color,      // Other users' processes (shadow)

    // Column-specific colors
    pub col_pid: Color,
    pub col_user: Color,
    pub col_priority: Color,
    pub col_mem_high: Color,       // highlight_megabytes color
    pub col_mem_normal: Color,
    pub col_cpu_high: Color,       // >75%
    pub col_cpu_medium: Color,     // >25%
    pub col_cpu_low: Color,        // <=25%
    pub col_status_running: Color,
    pub col_status_sleeping: Color,
    pub col_status_disk_sleep: Color,
    pub col_status_stopped: Color,
    pub col_status_zombie: Color,
    pub col_status_unknown: Color,
    pub col_command: Color,
    pub col_command_basename: Color, // Highlighted base name
    pub col_thread: Color,          // Thread color

    // Footer
    pub footer_key_fg: Color,
    pub footer_key_bg: Color,
    pub footer_label_fg: Color,
    pub footer_label_bg: Color,

    // Tab bar
    pub tab_active_bg: Color,
    pub tab_active_fg: Color,
    pub tab_inactive_fg: Color,
    pub tab_inactive_bg: Color,

    // Popups / menus
    pub popup_border: Color,
    pub popup_bg: Color,
    pub popup_title: Color,
    pub popup_selected_bg: Color,
    pub popup_selected_fg: Color,
    pub popup_text: Color,

    // Search/filter bar
    pub search_label: Color,
    pub search_text: Color,
    pub filter_label: Color,
    pub filter_text: Color,
}

impl ColorScheme {
    pub fn from_id(id: ColorSchemeId) -> Self {
        match id {
            ColorSchemeId::Default => Self::default_scheme(),
            ColorSchemeId::Monochrome => Self::monochrome(),
            ColorSchemeId::BlackNight => Self::black_night(),
            ColorSchemeId::LightTerminal => Self::light_terminal(),
            ColorSchemeId::MidnightCommander => Self::midnight_commander(),
            ColorSchemeId::BlackOnWhite => Self::black_on_white(),
            ColorSchemeId::DarkVivid => Self::dark_vivid(),
            ColorSchemeId::Cga => Self::cga(),
            ColorSchemeId::Campbell => Self::campbell(),
            ColorSchemeId::CampbellPowershell => Self::campbell_powershell(),
            ColorSchemeId::DarkPlus => Self::dark_plus(),
            ColorSchemeId::Dimidium => Self::dimidium(),
            ColorSchemeId::Ibm5153 => Self::ibm_5153(),
            ColorSchemeId::OneHalfDark => Self::one_half_dark(),
            ColorSchemeId::OneHalfLight => Self::one_half_light(),
            ColorSchemeId::Ottosson => Self::ottosson(),
            ColorSchemeId::SolarizedDark => Self::solarized_dark(),
            ColorSchemeId::SolarizedLight => Self::solarized_light(),
            ColorSchemeId::TangoDark => Self::tango_dark(),
            ColorSchemeId::TangoLight => Self::tango_light(),
            ColorSchemeId::Vintage => Self::vintage(),
            ColorSchemeId::VscodeDarkModern => Self::vscode_dark_modern(),
            ColorSchemeId::VscodeLightModern => Self::vscode_light_modern(),
        }
    }

    /// htop Default color scheme
    fn default_scheme() -> Self {
        Self {
            bg: Color::Reset,

            cpu_bar_normal: Color::Green,
            cpu_bar_system: Color::Red,
            cpu_bar_low: Color::Blue,
            cpu_bar_virt: Color::Cyan,
            cpu_bar_iowait: Color::DarkGray,
            cpu_bar_irq: Color::Yellow,
            cpu_bar_softirq: Color::Magenta,
            cpu_label: Color::White,
            cpu_bar_bg: Color::DarkGray,

            mem_bar_used: Color::Green,
            mem_bar_buffers: Color::Blue,
            mem_bar_cache: Color::Yellow,

            swap_bar: Color::Red,

            tasks_text: Color::White,
            load_text: Color::White,
            uptime_text: Color::White,
            info_label: Color::White,
            info_value: Color::Cyan,

            table_header_bg: Color::Cyan,
            table_header_fg: Color::Black,
            table_header_sort_bg: Color::Green,
            table_header_sort_fg: Color::Black,

            process_fg: Color::White,
            process_bg: Color::Reset,
            process_selected_bg: Color::Indexed(236),
            process_selected_fg: Color::White,
            process_shadow: Color::DarkGray,

            col_pid: Color::Green,
            col_user: Color::White,
            col_priority: Color::White,
            col_mem_high: Color::Green,
            col_mem_normal: Color::White,
            col_cpu_high: Color::Red,
            col_cpu_medium: Color::Yellow,
            col_cpu_low: Color::Green,
            col_status_running: Color::Green,
            col_status_sleeping: Color::Reset,
            col_status_disk_sleep: Color::Red,
            col_status_stopped: Color::Red,
            col_status_zombie: Color::Magenta,
            col_status_unknown: Color::DarkGray,
            col_command: Color::White,
            col_command_basename: Color::Green,
            col_thread: Color::Blue,

            footer_key_fg: Color::White,
            footer_key_bg: Color::Black,
            footer_label_fg: Color::Black,
            footer_label_bg: Color::Cyan,

            tab_active_bg: Color::Cyan,
            tab_active_fg: Color::Black,
            tab_inactive_fg: Color::DarkGray,
            tab_inactive_bg: Color::Indexed(235),

            popup_border: Color::Cyan,
            popup_bg: Color::Black,
            popup_title: Color::Cyan,
            popup_selected_bg: Color::Cyan,
            popup_selected_fg: Color::Black,
            popup_text: Color::White,

            search_label: Color::Cyan,
            search_text: Color::White,
            filter_label: Color::Yellow,
            filter_text: Color::White,
        }
    }

    /// Monochrome (no colors)
    fn monochrome() -> Self {
        Self {
            bg: Color::Reset,

            cpu_bar_normal: Color::White,
            cpu_bar_system: Color::White,
            cpu_bar_low: Color::White,
            cpu_bar_virt: Color::White,
            cpu_bar_iowait: Color::White,
            cpu_bar_irq: Color::White,
            cpu_bar_softirq: Color::White,
            cpu_label: Color::White,
            cpu_bar_bg: Color::DarkGray,

            mem_bar_used: Color::White,
            mem_bar_buffers: Color::White,
            mem_bar_cache: Color::White,

            swap_bar: Color::White,

            tasks_text: Color::White,
            load_text: Color::White,
            uptime_text: Color::White,
            info_label: Color::White,
            info_value: Color::White,

            table_header_bg: Color::White,
            table_header_fg: Color::Black,
            table_header_sort_bg: Color::White,
            table_header_sort_fg: Color::Black,

            process_fg: Color::White,
            process_bg: Color::Reset,
            process_selected_bg: Color::White,
            process_selected_fg: Color::Black,
            process_shadow: Color::DarkGray,

            col_pid: Color::White,
            col_user: Color::White,
            col_priority: Color::White,
            col_mem_high: Color::White,
            col_mem_normal: Color::White,
            col_cpu_high: Color::White,
            col_cpu_medium: Color::White,
            col_cpu_low: Color::White,
            col_status_running: Color::White,
            col_status_sleeping: Color::White,
            col_status_disk_sleep: Color::White,
            col_status_stopped: Color::White,
            col_status_zombie: Color::White,
            col_status_unknown: Color::DarkGray,
            col_command: Color::White,
            col_command_basename: Color::White,
            col_thread: Color::White,

            footer_key_fg: Color::White,
            footer_key_bg: Color::Black,
            footer_label_fg: Color::Black,
            footer_label_bg: Color::White,

            tab_active_bg: Color::White,
            tab_active_fg: Color::Black,
            tab_inactive_fg: Color::DarkGray,
            tab_inactive_bg: Color::Indexed(235),

            popup_border: Color::White,
            popup_bg: Color::Black,
            popup_title: Color::White,
            popup_selected_bg: Color::White,
            popup_selected_fg: Color::Black,
            popup_text: Color::White,

            search_label: Color::White,
            search_text: Color::White,
            filter_label: Color::White,
            filter_text: Color::White,
        }
    }

    /// Black Night — dark theme, similar to default but adjusted for dark terminals
    fn black_night() -> Self {
        Self {
            bg: Color::Black,

            cpu_bar_normal: Color::Green,
            cpu_bar_system: Color::Red,
            cpu_bar_low: Color::Indexed(33),   // Bright blue
            cpu_bar_virt: Color::Cyan,
            cpu_bar_iowait: Color::Indexed(245),
            cpu_bar_irq: Color::Yellow,
            cpu_bar_softirq: Color::Magenta,
            cpu_label: Color::Indexed(250),
            cpu_bar_bg: Color::Indexed(238),

            mem_bar_used: Color::Green,
            mem_bar_buffers: Color::Indexed(33),
            mem_bar_cache: Color::Yellow,

            swap_bar: Color::Red,

            tasks_text: Color::Indexed(250),
            load_text: Color::Indexed(250),
            uptime_text: Color::Indexed(250),
            info_label: Color::Indexed(250),
            info_value: Color::Cyan,

            table_header_bg: Color::Indexed(238),
            table_header_fg: Color::Indexed(250),
            table_header_sort_bg: Color::Green,
            table_header_sort_fg: Color::Black,

            process_fg: Color::Indexed(250),
            process_bg: Color::Black,
            process_selected_bg: Color::Indexed(236),
            process_selected_fg: Color::Indexed(250),
            process_shadow: Color::Indexed(240),

            col_pid: Color::Green,
            col_user: Color::Indexed(250),
            col_priority: Color::Indexed(250),
            col_mem_high: Color::Green,
            col_mem_normal: Color::Indexed(250),
            col_cpu_high: Color::Red,
            col_cpu_medium: Color::Yellow,
            col_cpu_low: Color::Green,
            col_status_running: Color::Green,
            col_status_sleeping: Color::Indexed(250),
            col_status_disk_sleep: Color::Red,
            col_status_stopped: Color::Red,
            col_status_zombie: Color::Magenta,
            col_status_unknown: Color::Indexed(240),
            col_command: Color::Indexed(250),
            col_command_basename: Color::Green,
            col_thread: Color::Indexed(33),

            footer_key_fg: Color::Cyan,
            footer_key_bg: Color::Black,
            footer_label_fg: Color::Black,
            footer_label_bg: Color::Green,

            tab_active_bg: Color::Cyan,
            tab_active_fg: Color::Black,
            tab_inactive_fg: Color::Indexed(240),
            tab_inactive_bg: Color::Indexed(234),

            popup_border: Color::Cyan,
            popup_bg: Color::Black,
            popup_title: Color::Cyan,
            popup_selected_bg: Color::Cyan,
            popup_selected_fg: Color::Black,
            popup_text: Color::Indexed(250),

            search_label: Color::Cyan,
            search_text: Color::Indexed(250),
            filter_label: Color::Yellow,
            filter_text: Color::Indexed(250),
        }
    }

    /// Light Terminal — for white/light backgrounds
    fn light_terminal() -> Self {
        Self {
            bg: Color::White,

            cpu_bar_normal: Color::Green,
            cpu_bar_system: Color::Red,
            cpu_bar_low: Color::Blue,
            cpu_bar_virt: Color::Cyan,
            cpu_bar_iowait: Color::DarkGray,
            cpu_bar_irq: Color::Yellow,
            cpu_bar_softirq: Color::Magenta,
            cpu_label: Color::Black,
            cpu_bar_bg: Color::Indexed(252),

            mem_bar_used: Color::Green,
            mem_bar_buffers: Color::Blue,
            mem_bar_cache: Color::Yellow,

            swap_bar: Color::Red,

            tasks_text: Color::Black,
            load_text: Color::Black,
            uptime_text: Color::Black,
            info_label: Color::Black,
            info_value: Color::Blue,

            table_header_bg: Color::Blue,
            table_header_fg: Color::White,
            table_header_sort_bg: Color::Green,
            table_header_sort_fg: Color::Black,

            process_fg: Color::Black,
            process_bg: Color::White,
            process_selected_bg: Color::Indexed(153),
            process_selected_fg: Color::Black,
            process_shadow: Color::Indexed(245),

            col_pid: Color::Blue,
            col_user: Color::Black,
            col_priority: Color::Black,
            col_mem_high: Color::Blue,
            col_mem_normal: Color::Black,
            col_cpu_high: Color::Red,
            col_cpu_medium: Color::Indexed(208),
            col_cpu_low: Color::Green,
            col_status_running: Color::Green,
            col_status_sleeping: Color::Black,
            col_status_disk_sleep: Color::Red,
            col_status_stopped: Color::Red,
            col_status_zombie: Color::Magenta,
            col_status_unknown: Color::Indexed(245),
            col_command: Color::Black,
            col_command_basename: Color::Blue,
            col_thread: Color::DarkGray,

            footer_key_fg: Color::White,
            footer_key_bg: Color::Blue,
            footer_label_fg: Color::Black,
            footer_label_bg: Color::White,

            tab_active_bg: Color::Blue,
            tab_active_fg: Color::White,
            tab_inactive_fg: Color::Indexed(245),
            tab_inactive_bg: Color::Indexed(254),

            popup_border: Color::Blue,
            popup_bg: Color::White,
            popup_title: Color::Blue,
            popup_selected_bg: Color::Blue,
            popup_selected_fg: Color::White,
            popup_text: Color::Black,

            search_label: Color::Blue,
            search_text: Color::Black,
            filter_label: Color::Indexed(208),
            filter_text: Color::Black,
        }
    }

    /// Midnight Commander style — blue background
    fn midnight_commander() -> Self {
        Self {
            bg: Color::Blue,

            cpu_bar_normal: Color::Green,
            cpu_bar_system: Color::Red,
            cpu_bar_low: Color::Cyan,
            cpu_bar_virt: Color::Yellow,
            cpu_bar_iowait: Color::DarkGray,
            cpu_bar_irq: Color::Yellow,
            cpu_bar_softirq: Color::Magenta,
            cpu_label: Color::White,
            cpu_bar_bg: Color::Indexed(17),

            mem_bar_used: Color::Green,
            mem_bar_buffers: Color::Cyan,
            mem_bar_cache: Color::Yellow,

            swap_bar: Color::Red,

            tasks_text: Color::White,
            load_text: Color::White,
            uptime_text: Color::White,
            info_label: Color::White,
            info_value: Color::Yellow,

            table_header_bg: Color::Cyan,
            table_header_fg: Color::Blue,
            table_header_sort_bg: Color::Yellow,
            table_header_sort_fg: Color::Blue,

            process_fg: Color::White,
            process_bg: Color::Blue,
            process_selected_bg: Color::Cyan,
            process_selected_fg: Color::Blue,
            process_shadow: Color::Indexed(67),

            col_pid: Color::Yellow,
            col_user: Color::White,
            col_priority: Color::White,
            col_mem_high: Color::Yellow,
            col_mem_normal: Color::White,
            col_cpu_high: Color::Red,
            col_cpu_medium: Color::Yellow,
            col_cpu_low: Color::Green,
            col_status_running: Color::Green,
            col_status_sleeping: Color::White,
            col_status_disk_sleep: Color::Red,
            col_status_stopped: Color::Red,
            col_status_zombie: Color::Magenta,
            col_status_unknown: Color::Indexed(67),
            col_command: Color::White,
            col_command_basename: Color::Yellow,
            col_thread: Color::Cyan,

            footer_key_fg: Color::White,
            footer_key_bg: Color::Blue,
            footer_label_fg: Color::Black,
            footer_label_bg: Color::Cyan,

            tab_active_bg: Color::Cyan,
            tab_active_fg: Color::Blue,
            tab_inactive_fg: Color::Indexed(67),
            tab_inactive_bg: Color::Blue,

            popup_border: Color::Yellow,
            popup_bg: Color::Blue,
            popup_title: Color::Yellow,
            popup_selected_bg: Color::Yellow,
            popup_selected_fg: Color::Blue,
            popup_text: Color::White,

            search_label: Color::Yellow,
            search_text: Color::White,
            filter_label: Color::Cyan,
            filter_text: Color::White,
        }
    }

    /// Black on White
    fn black_on_white() -> Self {
        Self {
            bg: Color::White,

            cpu_bar_normal: Color::Indexed(22),  // Dark green
            cpu_bar_system: Color::Indexed(124), // Dark red
            cpu_bar_low: Color::Indexed(25),     // Dark blue
            cpu_bar_virt: Color::Indexed(30),    // Dark cyan
            cpu_bar_iowait: Color::Indexed(245),
            cpu_bar_irq: Color::Indexed(136),
            cpu_bar_softirq: Color::Indexed(127),
            cpu_label: Color::Black,
            cpu_bar_bg: Color::Indexed(252),

            mem_bar_used: Color::Indexed(22),
            mem_bar_buffers: Color::Indexed(25),
            mem_bar_cache: Color::Indexed(130),

            swap_bar: Color::Indexed(124),

            tasks_text: Color::Black,
            load_text: Color::Black,
            uptime_text: Color::Black,
            info_label: Color::Black,
            info_value: Color::Indexed(25),

            table_header_bg: Color::Indexed(250),
            table_header_fg: Color::Black,
            table_header_sort_bg: Color::Indexed(22),
            table_header_sort_fg: Color::White,

            process_fg: Color::Black,
            process_bg: Color::White,
            process_selected_bg: Color::Indexed(250),
            process_selected_fg: Color::Black,
            process_shadow: Color::Indexed(245),

            col_pid: Color::Indexed(25),
            col_user: Color::Black,
            col_priority: Color::Black,
            col_mem_high: Color::Indexed(25),
            col_mem_normal: Color::Black,
            col_cpu_high: Color::Indexed(124),
            col_cpu_medium: Color::Indexed(130),
            col_cpu_low: Color::Indexed(22),
            col_status_running: Color::Indexed(22),
            col_status_sleeping: Color::Black,
            col_status_disk_sleep: Color::Indexed(124),
            col_status_stopped: Color::Indexed(124),
            col_status_zombie: Color::Indexed(127),
            col_status_unknown: Color::Indexed(245),
            col_command: Color::Black,
            col_command_basename: Color::Indexed(25),
            col_thread: Color::Indexed(245),

            footer_key_fg: Color::White,
            footer_key_bg: Color::Indexed(25),
            footer_label_fg: Color::Black,
            footer_label_bg: Color::White,

            tab_active_bg: Color::Indexed(25),
            tab_active_fg: Color::White,
            tab_inactive_fg: Color::Indexed(245),
            tab_inactive_bg: Color::Indexed(254),

            popup_border: Color::Indexed(25),
            popup_bg: Color::White,
            popup_title: Color::Indexed(25),
            popup_selected_bg: Color::Indexed(25),
            popup_selected_fg: Color::White,
            popup_text: Color::Black,

            search_label: Color::Indexed(25),
            search_text: Color::Black,
            filter_label: Color::Indexed(130),
            filter_text: Color::Black,
        }
    }

    /// Dark Vivid — high contrast, vivid colors on dark background
    fn dark_vivid() -> Self {
        Self {
            bg: Color::Black,

            cpu_bar_normal: Color::Indexed(46),   // Bright green
            cpu_bar_system: Color::Indexed(196),   // Bright red
            cpu_bar_low: Color::Indexed(39),       // Bright blue
            cpu_bar_virt: Color::Indexed(51),      // Bright cyan
            cpu_bar_iowait: Color::Indexed(245),
            cpu_bar_irq: Color::Indexed(226),      // Bright yellow
            cpu_bar_softirq: Color::Indexed(201),    // Bright magenta
            cpu_label: Color::White,
            cpu_bar_bg: Color::Indexed(235),

            mem_bar_used: Color::Indexed(46),
            mem_bar_buffers: Color::Indexed(39),
            mem_bar_cache: Color::Indexed(226),

            swap_bar: Color::Indexed(196),

            tasks_text: Color::White,
            load_text: Color::White,
            uptime_text: Color::White,
            info_label: Color::White,
            info_value: Color::Indexed(51),

            table_header_bg: Color::Indexed(236),
            table_header_fg: Color::Indexed(51),
            table_header_sort_bg: Color::Indexed(46),
            table_header_sort_fg: Color::Black,

            process_fg: Color::Indexed(252),
            process_bg: Color::Black,
            process_selected_bg: Color::Indexed(236),
            process_selected_fg: Color::Indexed(226),
            process_shadow: Color::Indexed(240),

            col_pid: Color::Indexed(46),
            col_user: Color::Indexed(252),
            col_priority: Color::Indexed(252),
            col_mem_high: Color::Indexed(46),
            col_mem_normal: Color::Indexed(252),
            col_cpu_high: Color::Indexed(196),
            col_cpu_medium: Color::Indexed(226),
            col_cpu_low: Color::Indexed(46),
            col_status_running: Color::Indexed(46),
            col_status_sleeping: Color::Indexed(252),
            col_status_disk_sleep: Color::Indexed(196),
            col_status_stopped: Color::Indexed(196),
            col_status_zombie: Color::Indexed(201),
            col_status_unknown: Color::Indexed(240),
            col_command: Color::Indexed(252),
            col_command_basename: Color::Indexed(46),
            col_thread: Color::Indexed(39),

            footer_key_fg: Color::Black,
            footer_key_bg: Color::Indexed(51),
            footer_label_fg: Color::Indexed(252),
            footer_label_bg: Color::Black,

            tab_active_bg: Color::Indexed(51),
            tab_active_fg: Color::Black,
            tab_inactive_fg: Color::Indexed(240),
            tab_inactive_bg: Color::Indexed(233),

            popup_border: Color::Indexed(51),
            popup_bg: Color::Black,
            popup_title: Color::Indexed(51),
            popup_selected_bg: Color::Indexed(51),
            popup_selected_fg: Color::Black,
            popup_text: Color::Indexed(252),

            search_label: Color::Indexed(51),
            search_text: Color::Indexed(252),
            filter_label: Color::Indexed(226),
            filter_text: Color::Indexed(252),
        }
    }

    // ── Windows Terminal palette import ─────────────────────────────────
    //
    // Windows Terminal color schemes are defined as a 16 color ANSI palette
    // plus separate foreground/background colors. We map that onto our
    // roughly 60 color slots using a fixed semantic rule: green = normal/ok,
    // red = system/high, yellow = medium, cyan = accent/chrome, blue =
    // selection background. Note that in Windows Terminal JSON the "purple"
    // field is what we call magenta everywhere else in this file.

    /// Parse a "#RRGGBB" hex string into a `Color::Rgb`.
    fn hex(s: &str) -> Color {
        let s = s.trim_start_matches('#');
        let r = u8::from_str_radix(&s[0..2], 16).unwrap();
        let g = u8::from_str_radix(&s[2..4], 16).unwrap();
        let b = u8::from_str_radix(&s[4..6], 16).unwrap();
        Color::Rgb(r, g, b)
    }

    /// Build a full ColorScheme from a Windows Terminal 16 color palette.
    /// The parameter order mirrors the scheme layout in Windows Terminal JSON.
    #[allow(clippy::too_many_arguments)]
    fn from_wt(
        bg: &str,
        fg: &str,
        black: &str,
        red: &str,
        green: &str,
        yellow: &str,
        blue: &str,
        purple: &str,
        cyan: &str,
        bright_black: &str,
        bright_white: &str,
    ) -> Self {
        let bg = Self::hex(bg);
        let fg = Self::hex(fg);
        let black = Self::hex(black);
        let red = Self::hex(red);
        let green = Self::hex(green);
        let yellow = Self::hex(yellow);
        let blue = Self::hex(blue);
        let purple = Self::hex(purple);
        let cyan = Self::hex(cyan);
        let bright_black = Self::hex(bright_black);
        let bright_white = Self::hex(bright_white);

        Self {
            bg,

            cpu_bar_normal: green,
            cpu_bar_system: red,
            cpu_bar_low: blue,
            cpu_bar_virt: cyan,
            cpu_bar_iowait: bright_black,
            cpu_bar_irq: yellow,
            cpu_bar_softirq: purple,
            cpu_label: fg,
            cpu_bar_bg: bright_black,

            mem_bar_used: green,
            mem_bar_buffers: blue,
            mem_bar_cache: yellow,

            swap_bar: red,

            tasks_text: fg,
            load_text: fg,
            uptime_text: fg,
            info_label: fg,
            info_value: cyan,

            table_header_bg: cyan,
            table_header_fg: black,
            table_header_sort_bg: green,
            table_header_sort_fg: black,

            process_fg: fg,
            process_bg: bg,
            process_selected_bg: blue,
            process_selected_fg: bright_white,
            process_shadow: bright_black,

            col_pid: green,
            col_user: fg,
            col_priority: fg,
            col_mem_high: green,
            col_mem_normal: fg,
            col_cpu_high: red,
            col_cpu_medium: yellow,
            col_cpu_low: green,
            col_status_running: green,
            col_status_sleeping: fg,
            col_status_disk_sleep: red,
            col_status_stopped: red,
            col_status_zombie: purple,
            col_status_unknown: bright_black,
            col_command: fg,
            col_command_basename: green,
            col_thread: blue,

            footer_key_fg: fg,
            footer_key_bg: bg,
            footer_label_fg: black,
            footer_label_bg: cyan,

            tab_active_bg: cyan,
            tab_active_fg: black,
            tab_inactive_fg: bright_black,
            tab_inactive_bg: bg,

            popup_border: cyan,
            popup_bg: bg,
            popup_title: cyan,
            popup_selected_bg: cyan,
            popup_selected_fg: black,
            popup_text: fg,

            search_label: cyan,
            search_text: fg,
            filter_label: yellow,
            filter_text: fg,
        }
    }

    /// Windows Terminal CGA
    fn cga() -> Self {
        Self::from_wt(
            "#000000", "#AAAAAA", "#000000", "#AA0000", "#00AA00", "#AA5500", "#0000AA",
            "#AA00AA", "#00AAAA", "#555555", "#FFFFFF",
        )
    }

    /// Windows Terminal Campbell (default dark scheme)
    fn campbell() -> Self {
        Self::from_wt(
            "#0C0C0C", "#CCCCCC", "#0C0C0C", "#C50F1F", "#13A10E", "#C19C00", "#0037DA",
            "#881798", "#3A96DD", "#767676", "#F2F2F2",
        )
    }

    /// Windows Terminal Campbell PowerShell
    fn campbell_powershell() -> Self {
        Self::from_wt(
            "#012456", "#CCCCCC", "#0C0C0C", "#C50F1F", "#13A10E", "#C19C00", "#0037DA",
            "#881798", "#3A96DD", "#767676", "#F2F2F2",
        )
    }

    /// Windows Terminal Dark+
    fn dark_plus() -> Self {
        Self::from_wt(
            "#1E1E1E", "#CCCCCC", "#000000", "#CD3131", "#0DBC79", "#E5E510", "#2472C8",
            "#BC3FBC", "#11A8CD", "#666666", "#E5E5E5",
        )
    }

    /// Windows Terminal Dimidium
    fn dimidium() -> Self {
        Self::from_wt(
            "#141414", "#BAB7B6", "#000000", "#CF494C", "#60B442", "#DB9C11", "#0575D8",
            "#AF5ED2", "#1DB6BB", "#817E7E", "#DEE3E4",
        )
    }

    /// Windows Terminal IBM 5153
    fn ibm_5153() -> Self {
        Self::from_wt(
            "#000000", "#AAAAAA", "#000000", "#AA0000", "#00AA00", "#C47E00", "#0000AA",
            "#AA00AA", "#00AAAA", "#555555", "#FFFFFF",
        )
    }

    /// Windows Terminal One Half Dark
    fn one_half_dark() -> Self {
        Self::from_wt(
            "#282C34", "#DCDFE4", "#282C34", "#E06C75", "#98C379", "#E5C07B", "#61AFEF",
            "#C678DD", "#56B6C2", "#5A6374", "#DCDFE4",
        )
    }

    /// Windows Terminal One Half Light
    fn one_half_light() -> Self {
        Self::from_wt(
            "#FAFAFA", "#383A42", "#383A42", "#E45649", "#50A14F", "#C18301", "#0184BC",
            "#A626A4", "#0997B3", "#4F525D", "#FFFFFF",
        )
    }

    /// Windows Terminal Ottosson
    fn ottosson() -> Self {
        Self::from_wt(
            "#000000", "#BEBEBE", "#000000", "#BE2C21", "#3FAE3A", "#BE9A4A", "#204DBE",
            "#BB54BE", "#00A7B2", "#808080", "#FFFFFF",
        )
    }

    /// Windows Terminal Solarized Dark
    fn solarized_dark() -> Self {
        Self::from_wt(
            "#002B36", "#839496", "#002B36", "#DC322F", "#859900", "#B58900", "#268BD2",
            "#D33682", "#2AA198", "#073642", "#FDF6E3",
        )
    }

    /// Windows Terminal Solarized Light
    fn solarized_light() -> Self {
        Self::from_wt(
            "#FDF6E3", "#657B83", "#002B36", "#DC322F", "#859900", "#B58900", "#268BD2",
            "#D33682", "#2AA198", "#073642", "#FDF6E3",
        )
    }

    /// Windows Terminal Tango Dark
    fn tango_dark() -> Self {
        Self::from_wt(
            "#000000", "#D3D7CF", "#000000", "#CC0000", "#4E9A06", "#C4A000", "#3465A4",
            "#75507B", "#06989A", "#555753", "#EEEEEC",
        )
    }

    /// Windows Terminal Tango Light
    fn tango_light() -> Self {
        Self::from_wt(
            "#FFFFFF", "#555753", "#000000", "#CC0000", "#4E9A06", "#C4A000", "#3465A4",
            "#75507B", "#06989A", "#555753", "#EEEEEC",
        )
    }

    /// Windows Terminal Vintage
    fn vintage() -> Self {
        Self::from_wt(
            "#000000", "#C0C0C0", "#000000", "#800000", "#008000", "#808000", "#000080",
            "#800080", "#008080", "#808080", "#FFFFFF",
        )
    }

    /// Windows Terminal VSCode Dark Modern
    fn vscode_dark_modern() -> Self {
        Self::from_wt(
            "#1F1F1F", "#CCCCCC", "#000000", "#CD3131", "#0DBC79", "#E5E510", "#2472C8",
            "#BC3FBC", "#11A8CD", "#666666", "#E5E5E5",
        )
    }

    /// Windows Terminal VSCode Light Modern
    fn vscode_light_modern() -> Self {
        Self::from_wt(
            "#FFFFFF", "#3B3B3B", "#000000", "#CD3131", "#00BC00", "#949800", "#0451A5",
            "#BC05BC", "#0598BC", "#666666", "#A5A5A5",
        )
    }

    // ── Convenience style builders ──────────────────────────────────────

    pub fn header_normal_style(&self) -> Style {
        Style::default().fg(self.cpu_bar_normal)
    }

    pub fn header_system_style(&self) -> Style {
        Style::default().fg(self.cpu_bar_system)
    }

    pub fn table_header_style(&self) -> Style {
        Style::default().fg(self.table_header_fg).bg(self.table_header_bg)
    }

    pub fn table_header_sort_style(&self) -> Style {
        Style::default()
            .fg(self.table_header_sort_fg)
            .bg(self.table_header_sort_bg)
            .add_modifier(Modifier::BOLD)
    }

    pub fn process_style(&self) -> Style {
        Style::default().fg(self.process_fg).bg(self.process_bg)
    }

    pub fn selected_style(&self) -> Style {
        Style::default().fg(self.process_selected_fg).bg(self.process_selected_bg)
    }

    pub fn footer_key_style(&self) -> Style {
        Style::default().fg(self.footer_key_fg).bg(self.footer_key_bg)
    }

    pub fn footer_label_style(&self) -> Style {
        Style::default().fg(self.footer_label_fg).bg(self.footer_label_bg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_contains_23_schemes() {
        assert_eq!(ColorSchemeId::all().len(), 23);
    }

    #[test]
    fn from_index_round_trips_for_every_scheme() {
        for i in 0..ColorSchemeId::all().len() {
            assert_eq!(ColorSchemeId::from_index(i) as usize, i, "index {} does not round trip", i);
        }
    }

    #[test]
    fn from_index_out_of_range_falls_back_to_default() {
        assert_eq!(ColorSchemeId::from_index(999), ColorSchemeId::Default);
        assert_eq!(ColorSchemeId::from_index(23), ColorSchemeId::Default);
    }

    #[test]
    fn every_scheme_builds_with_nonempty_name_and_description() {
        for id in ColorSchemeId::all() {
            let _scheme = ColorScheme::from_id(*id);
            assert!(!id.name().is_empty());
            assert!(!id.description().is_empty());
        }
    }

    #[test]
    fn scheme_names_are_unique() {
        let mut names: Vec<&str> = ColorSchemeId::all().iter().map(|id| id.name()).collect();
        let total = names.len();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), total);
    }

    #[test]
    fn windows_terminal_schemes_use_exact_palette_backgrounds() {
        let cases = [
            (ColorSchemeId::Campbell, Color::Rgb(0x0C, 0x0C, 0x0C)),
            (ColorSchemeId::CampbellPowershell, Color::Rgb(0x01, 0x24, 0x56)),
            (ColorSchemeId::SolarizedDark, Color::Rgb(0x00, 0x2B, 0x36)),
            (ColorSchemeId::SolarizedLight, Color::Rgb(0xFD, 0xF6, 0xE3)),
            (ColorSchemeId::OneHalfDark, Color::Rgb(0x28, 0x2C, 0x34)),
            (ColorSchemeId::VscodeDarkModern, Color::Rgb(0x1F, 0x1F, 0x1F)),
        ];
        for (id, expected_bg) in cases {
            assert_eq!(ColorScheme::from_id(id).bg, expected_bg, "wrong bg for {}", id.name());
        }
    }
}
