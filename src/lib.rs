#[macro_use]
extern crate error_chain;
mod errors {
    error_chain! { }
}

#[macro_use] 
extern crate serde_derive;

use self::errors::*;

// TODO: Also add units for bits, bps, kbps etc
// TODO: in the constructors check that incoming float does not have excessive precision. The precision should not be such that one converted to bytes we get a fractional value
// TODO: think of how the u64 in Bytes can overflow when converting down from higher units (like KiB) and how to guard against it
// One of option with the above two point is to silently truncate. That'll make the API simpler, but will it be safe enough for the user? Is it enough if we just mention lossiness in the docs?
// TODO: Provide and additional way to automatically choose the most convenient unit while displaying. For instance 512 bytes should display as "512 Bytes" but 1024 bytes should display as "1 KiB"
// TODO: Add support for parsing these types from strings as well
// TODO: Impl mathematical operators on these types using newtype_derive

macro_rules! impl_conv {
    ($type_from:tt, $type_to:tt, $multiple:expr) => {
        impl From<$type_from> for $type_to {
            fn from(from_obj: $type_from) -> Self {
                $type_to(from_obj.0 as f64 * $multiple)
            }
        }       
    };
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bytes(pub u64);
pub type B = Bytes;

#[derive(Serialize, Deserialize, Debug)]
pub struct kB(pub f64);

impl_conv!(Bytes, kB, 1_f64 / 1000_f64);

#[derive(Serialize, Deserialize, Debug)]
pub struct MB(pub f64);

impl_conv!(Bytes, MB, 1_f64 / 1000_000_f64);

#[derive(Serialize, Deserialize, Debug)]
pub struct GB(pub f64);

impl_conv!(Bytes, GB, 1_f64 / 1000_000_000_f64);

#[derive(Serialize, Deserialize, Debug)]
pub struct KiB(pub f64);

impl_conv!(Bytes, KiB, 1_f64 / 1024_f64);

#[derive(Serialize, Deserialize, Debug)]
pub struct MiB(f64);

impl_conv!(Bytes, MiB, 1_f64 / 1048_576_f64);

#[derive(Serialize, Deserialize, Debug)]
pub struct GiB(f64);

impl_conv!(Bytes, GiB, 1_f64 / 1073_741_824_f64);
