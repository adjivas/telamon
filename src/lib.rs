extern crate boxfnonce;
extern crate config;
extern crate crossbeam;
extern crate data_structure_traits;
#[cfg(test)]
extern crate env_logger;
extern crate errno;
extern crate getopts;
extern crate immut_list;
extern crate interval_heap;
#[cfg(feature = "cuda")]
extern crate ipc_channel;
extern crate itertools;
#[cfg(feature = "cuda")]
#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate linked_list;
#[macro_use]
extern crate log;
#[macro_use]
extern crate matches;
extern crate futures;
extern crate num;
#[cfg(feature = "mppa")]
extern crate parking_lot;
#[cfg(feature = "cuda")]
extern crate prctl;
extern crate rand;
#[cfg(feature = "cuda")]
extern crate rustc_serialize;
extern crate tokio_timer;
#[macro_use]
extern crate telamon_utils as utils;

pub mod codegen;
#[macro_use]
pub mod helper;
pub mod device;
pub mod explorer;
pub mod ir;
pub mod model;
pub mod search_space;
