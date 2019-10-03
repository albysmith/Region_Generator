use rand::seq::SliceRandom;
use rand::Rng;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
#[macro_use]
extern crate serde;
extern crate serde_xml_rs;

#[derive(Deserialize, Debug, Default, Clone)]
struct Toml {
    config: Config,

}

#[derive(Deserialize, Debug, Default, Clone)]
struct Config {

}


/*

To make a region:
-

*/ 

fn main() {
    println!("Hello, world!");
}
