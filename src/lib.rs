#[macro_use]
extern crate error_chain;
mod errors {
    error_chain! { }
}

use self::errors::*;

// TODO: Also add units for bits, bps, kbps etc

macro_rules! impl_conv {
    ($type_from:tt, $type_to:tt, $multiple:expr) => {
        impl From<$type_from> for $type_to {
            fn from(from_obj: $type_from) -> Self {
                $type_to(from_obj.0 as f64 * $multiple)
            }
        }       
    };
}

pub struct Bytes(pub u64);
pub type B = Bytes;

pub struct kB(pub f64);

impl_conv!(Bytes, kB, 1.0 / 1000.0);

pub struct MB(pub f64);

impl_conv!(Bytes, MB, 1.0 / 1000_000.0);

pub struct GB(pub f64);

impl_conv!(Bytes, GB, 1.0 / 1000_000_000.0);


pub struct KiB(pub f64);

impl_conv!(Bytes, KiB, 1.0 / 1024.0);

pub struct MiB(f64);

impl_conv!(Bytes, MiB, 1.0 / 1048_576.0);

pub struct GiB(f64);

impl_conv!(Bytes, GiB, 1.0 / 1073_741_824.0);
