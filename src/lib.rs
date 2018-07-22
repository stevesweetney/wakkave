#[cfg(feature = "frontend")]
#[macro_use]
extern crate yew;

#[cfg(feature="default")]
extern crate actix;

#[cfg(feature="default")]
extern crate actix_web;

#[cfg(feature="default")]
pub mod backend;

#[cfg(feature = "frontend")]
pub mod frontend;