pub(crate) mod internal;

pub mod api;
pub mod constants;
pub mod enums;
pub mod models;
pub mod util;

#[cfg(feature = "socket")]
pub mod socket;

#[cfg(feature = "static-http-client")]
#[macro_use]
extern crate lazy_static;
