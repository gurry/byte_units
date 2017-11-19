#[macro_use]
extern crate error_chain;
mod errors {
    error_chain! { }
}

use self::errors::*;

// TODO: Replace all explicit From impls with a rust macro which takes a conversion multiple and the two types involved
// TODO: also add units for bits, bps, kbps etc

pub struct Bytes(pub u64);
pub type B = Bytes;

pub struct kB(pub f64);
pub struct MB(pub f64);
pub struct GB(pub f64);

pub struct KiB(pub f64);

impl From<Bytes> for KiB {
    fn from(bytes: Bytes) -> Self {
        KiB(bytes.0 as f64 / 1024.0)
    }
}

pub struct MiB(f64);

impl From<Bytes> for MiB {
    fn from(bytes: Bytes) -> Self {
        MiB(bytes.0 as f64 / 1048576.0)
    }
}

pub struct GiB(f64);

impl From<Bytes> for GiB {
    fn from(bytes: Bytes) -> Self {
        GiB(bytes.0 as f64 / 1073741824.0)
    }
}
