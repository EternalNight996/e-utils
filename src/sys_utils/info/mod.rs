mod mem;
mod monitor;
use mem::MemInfo;
use monitor::MonitorInfo;

/// Info
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Info {
    /// Operating system edition.
    pub(crate) edition: Option<String>,
    pub(crate) monitor: MonitorInfo,
    pub(crate) mem: MemInfo,
}
impl Info {
    /// new info
    pub fn new() -> Self {
        Self {
            edition: None,
            monitor: MonitorInfo::new(),
            mem: MemInfo::new(),
        }
    }
}
impl Info {
    /// get_monitor
    pub fn get_monitor(&self) -> MonitorInfo {
        self.monitor.clone()
    }
    /// get_mem
    pub fn get_mem(&self) -> MemInfo {
        self.mem.clone()
    }
}
