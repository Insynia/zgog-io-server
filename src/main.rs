//! Server for zgog-io game.

#![feature(custom_attribute)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_repr;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

pub mod communication;
pub mod coordinates;
pub mod map;
pub mod players;

use crate::communication::launch_server;

fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    launch_server();
}
