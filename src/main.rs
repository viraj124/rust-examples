
use core::prelude;
use std::fmt::Write;
use std::io;
use std::cmp::Ordering;

pub mod random;
pub mod control_flow;
mod ownership;
mod slicing;
mod structs;
mod enums;


fn main() {
    random::checkRandom();
    control_flow::flow();
    ownership::ownership();
    slicing::slice();
    structs::structs();
    enums::enums();
}


