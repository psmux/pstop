/// Per-core CPU usage info
#[derive(Debug, Clone, Default)]
pub struct CpuCore {
    pub id: usize,
    pub usage_percent: f32,
    pub frequency_mhz: u64,
    // Per-core time fractions (of total time including idle), for htop-style bar segments
    pub user_frac: f32,      // user mode (htop: green)
    pub kernel_frac: f32,    // pure kernel (htop: red)
    pub dpc_frac: f32,       // DPC time ≈ softirq (htop: magenta)
    pub interrupt_frac: f32, // interrupt time ≈ irq (htop: yellow)
}

/// Aggregate CPU information
#[derive(Debug, Clone, Default)]
pub struct CpuInfo {
    pub cores: Vec<CpuCore>,
    pub total_usage: f32,
    pub physical_cores: usize,
    pub logical_cores: usize,
    pub brand: String,
}

impl CpuInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
