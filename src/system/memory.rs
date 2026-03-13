/// Memory usage information
#[derive(Debug, Clone, Default)]
pub struct MemoryInfo {
    pub total_mem: u64,      // bytes
    pub used_mem: u64,       // bytes
    pub free_mem: u64,       // bytes
    pub cached_mem: u64,     // bytes
    pub buffered_mem: u64,   // bytes (not separated on Windows)
    pub total_swap: u64,     // bytes
    pub used_swap: u64,      // bytes
    pub free_swap: u64,      // bytes
}

impl MemoryInfo {
    pub fn new() -> Self {
        Self::default()
    }

    /// Memory usage as percentage
    pub fn mem_percent(&self) -> f64 {
        if self.total_mem == 0 {
            0.0
        } else {
            (self.used_mem as f64 / self.total_mem as f64) * 100.0
        }
    }

    /// Swap usage as percentage
    pub fn swap_percent(&self) -> f64 {
        if self.total_swap == 0 {
            0.0
        } else {
            (self.used_swap as f64 / self.total_swap as f64) * 100.0
        }
    }
}

/// Format bytes to human-readable string matching htop's Meter_humanUnit exactly.
/// Adaptive precision: 2 decimals for <10, 1 decimal for <100, 0 for ≥100.
pub fn format_bytes(bytes: u64) -> String {
    const ONE_K: f64 = 1024.0;
    let prefixes = ['K', 'M', 'G', 'T', 'P'];

    let mut value = bytes as f64 / ONE_K; // convert to KiB first (htop base unit)
    let mut i: usize = 0;

    if bytes < 1024 {
        return format!("{}B", bytes);
    }

    while value >= ONE_K && i < prefixes.len() - 1 {
        value /= ONE_K;
        i += 1;
    }

    // htop's adaptive precision: ≤9.99 → 2 dec, ≤99.9 → 1 dec, else 0 dec
    let precision = if i == 0 {
        0 // KiB: no decimals (htop: raw KiB count)
    } else if value <= 9.99 {
        2
    } else if value <= 99.9 {
        1
    } else {
        0
    };

    format!("{:.*}{}", precision, value, prefixes[i])
}
