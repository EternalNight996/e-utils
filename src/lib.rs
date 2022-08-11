//! A RUST UTILS
//! # Quick start
//! ```
//! fn nanoid() {
//!   use e_utils::random;
//!   println!("nanoid -> {}", random!(nanoid));
//!   println!("nanoid 16bytes -> {}", random!(nanoid 16));
//!   println!("nanoid 16bytes -> {}", random!(nanoid 16));
//!   println!(
//!       "nanoid 10bytes [alphabet:expr] -> {}",
//!       random!(nanoid 16, &['1', 'b', 'c', '7'])
//!   );
//!   println!("nanoid unsafe 10bytes -> {}", random!(nanoid unsafe 10));
//!   println!(
//!       "nanoid unsafe 10bytes [alphabet:expr]-> {}",
//!       random!(nanoid unsafe 10, &['1','0'])
// !   );
//!  }
//!  fn std() {
//!   use e_utils::random;
//!   println!("random bool -> {}", random!());
//!   println!("random type -> {}", random!(#u32));
//!   println!("random type[] -> {:?}", random!([u32; 10]));
//!   println!("random range 0-13 -> {}", random!(13i64));
//!   println!("random range -> {}", random!(0i32..13));
//!   println!("random rgb range -> {:?}", random!(rgb 100,255));
//!  }
//!  fn main() {
//!   nanoid();
//!   std();
//!  }
//!```

#![doc(
    html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png",
    html_favicon_url = "https://www.rust-lang.org/favicon.ico",
    html_root_url = "https://github.com/EternalNight996",
    issue_tracker_base_url = "https://github.com/EternalNight996/e-libscanner/issues/"
)]
#![warn(
    missing_debug_implementations,
    missing_doc_code_examples,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub,
    bad_style,
    const_err,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true
)]
#![doc(test(attr(allow(unused_variables), deny(warnings))))]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
// Rustc lints.
#![deny(missing_docs, unused_imports)]

#[macro_use]
mod cfgs;
cfg_random! {
    #[doc(hidden)]
    pub mod random;
    pub use rand;
}

cfg_std! {
    #[macro_use]
    pub mod p_std;
}

/// base64
#[cfg(feature = "base64")]
#[cfg_attr(docsrs, doc(cfg(feature = "base64")))]
pub mod base64;

cfg_dns! {
    /// dns
    #[path="sys_utils/dns/mod.rs"]
    pub mod dns;
}
cfg_traceroute! {
    /// traceroute
    #[path="sys_utils/traceroute/mod.rs"]
    pub mod traceroute;
}
cfg_sysinfo! {
    /// system info
    #[path="sys_utils/info/mod.rs"]
    pub mod info;
}
