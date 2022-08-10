#[macro_use]
mod cfgs;
cfg_random! {
    pub mod random;
    pub use rand;
}

cfg_std! {
    #[macro_use]
    pub mod p_std;
}

#[cfg(feature = "base64")]
#[cfg_attr(docsrs, doc(cfg(feature = "base64")))]
pub mod base64;

mod sys_utils;
pub use sys_utils::*;
