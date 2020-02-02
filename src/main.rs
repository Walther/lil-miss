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
            Arg::with_name("input")
                .short('i')
                .long("input")
                .value_name("INPUT")
                .help("Sets a custom input file")
                .takes_value(true),
        )
        .get_matches();

    if matches.is_present("debug") {
        pretty_env_logger::formatted_builder()
            .filter_level(log::LevelFilter::Debug)
            .init()
    }

    if let Some(filename) = matches.value_of("INPUT") {
        debug!("Reading file: {}", filename);
        let contents = load_file(filename)?;
        debug!("Tile: \n{}", contents.clone());
        let mut tile = load_tile(contents)?;
        let valid: Status = validate_tile(&tile)?;
        println!("{}", valid);
        return Ok(()); // early exit
    }

    let mut trues = 0_u32;
    let mut falses = 0_u32;
    let max = 2_u32.pow(21); // 21 booleans, 2**21 possible states
    let mut binary;
    let mut tile: Tile = Tile::new();
    for i in 0..max {
        let filename = format!("{:0>7}.txt", i); // pad to 7 numbers wide, because we're running to 2M which has seven digits

        // Do some bit magic. Convert number to binary, pad to width of 21 plus leading 0b = 23
        binary = format!("{:#023b}", i);
        let tilestring = &binary[2..]; // strip leading 0b
        let tilestring = str::replace(&tilestring, "0", ".");
        let tilestring = str::replace(&tilestring, "1", "#");
        tile = load_tile_binarystring(&mut tile, &tilestring)?;
        match validate_tile(&tile)? {
            Status::True => {
                trues += 1;
            }
            Status::False => {
                falses += 1;
            }
        }
        // generate_tile_file(&tile, &filename)?;
    }
    println!("Trues: {}", trues);
    println!("Falses: {}", falses);

    Ok(())
}

fn load_file(filename: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(filename)?;

    let mut buf_reader: BufReader<File> = BufReader::new(file);
    let mut contents: String = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn generate_tile_file(tile: &Tile, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut path = env::current_dir()?;
    path.push("tiles");
    path.push(filename);

    let mut file = File::create(path)?;
    write!(file, "{}", tile)?;
    Ok(())
}
