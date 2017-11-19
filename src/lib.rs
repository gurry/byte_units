#[macro_use]
extern crate error_chain;
mod errors {
    error_chain! { }
}

use self::errors::*;

pub struct Bytes(u64);
pub type B = Bytes;


pub struct KiB(u64);
pub struct MiB(u64);
pub struct GiB(u64);