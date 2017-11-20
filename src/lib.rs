#[macro_use]
extern crate error_chain;
mod errors {
    error_chain! { }
}

use self::errors::*;

// TODO: Also add units for bits, bps, kbps etc
// TODO: in the constructors check that incoming float does not have excessive precision. The precision should not be such that one converted to bytes we get a fractional value
// TODO: think of how the u64 in Bytes can overflow when converting down from higher units (like KiB) and how to guard against it
// One of option with the above two point is to silently truncate. That'll make the API simpler, but will it be safe enough for the user? Is it enough if we just mention lossiness in the docs?

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
