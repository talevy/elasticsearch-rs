//#![allow(dead_code, unused_imports)]
#![feature(collections)]
#![feature(std_misc)]
#![feature(plugin)]
#![feature(trace_macros, log_syntax)]
#![plugin(regex_macros)]

extern crate hyper;
extern crate rustc_serialize;
extern crate url;
extern crate chrono;
extern crate time;
extern crate collections;
extern crate regex;

#[macro_use]
mod macros;

mod types;
mod connection;
pub mod actions;
pub mod indices;
pub mod client;

