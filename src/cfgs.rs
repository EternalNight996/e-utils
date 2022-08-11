/// The `cfg_debug`
macro_rules! cfg_std {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "std")]
            #[cfg_attr(doc_cfg, doc(cfg(feature = "std")))]
            $item
        )*
    }
}
/// The `cfg_random`
macro_rules! cfg_random {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "random")]
            #[cfg_attr(doc_cfg, doc(cfg(feature = "random")))]
            $item
        )*
    }
}
/// The `cfg_sys_info`
macro_rules! cfg_sysinfo {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "sys_info")]
            #[cfg_attr(doc_cfg, doc(cfg(feature = "sys_info")))]
            $item
        )*
    }
}
/// The `cfg_dns`
macro_rules! cfg_dns {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "dns")]
            #[cfg_attr(doc_cfg, doc(cfg(feature = "dns")))]
            $item
        )*
    }
}
/// The `cfg_traceroute`
macro_rules! cfg_traceroute {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "traceroute")]
            #[cfg_attr(doc_cfg, doc(cfg(feature = "traceroute")))]
            $item
        )*
    }
}