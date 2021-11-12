//! Low-level bindings to [clockkit](https://github.com/camilleg/clockkit)
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub type dur = i64;
pub type tp = i64;

#[cfg(all(
    target_os = "linux",
    target_arch = "x86_64",
    not(feature = "make_bindings")
))]
include!("bindgen/amd64_unknown_linux_default");

#[cfg(feature = "make_bindings")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
