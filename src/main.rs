use std::collections::HashMap;
use std::error::Error;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
extern crate clap;
use clap::{App, Arg};
use std::env;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

pub mod lib;
use lib::*;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("lil-miss")
        .version("0.1")
        .about("lil tool for MIS uses")
        .author("Veeti 'Walther' Haapsamo")
        .arg("-d, --debug 'Turn debugging information on'")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    if matches.is_present("debug") {
        pretty_env_logger::formatted_builder()
            .filter_level(log::LevelFilter::Debug)
            .init()
    }

    let filename: &str = matches.value_of("INPUT").unwrap(); // how to make the error type match
    debug!("Reading file: {}", filename);
    let contents = load_file(filename)?;
    debug!("Tile: \n{}", contents.clone());
    let tile = load_tile(contents)?;
    let valid: Status = validate_tile(&tile)?;
    println!("{}", valid);
    Ok(())
}

fn load_file(filename: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(filename)?;

    let mut buf_reader: BufReader<File> = BufReader::new(file);
    let mut contents: String = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}
