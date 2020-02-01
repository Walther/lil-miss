use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{Error as ioError, ErrorKind};
extern crate clap;
use clap::{App, Arg};

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[derive(std::cmp::PartialEq)]
enum Square {
    False,
    True,
    Unknown,
}

type Tile = HashMap<(usize, usize), Square>;

#[derive(std::cmp::PartialEq)]
enum Status {
    False,
    True,
}

impl Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::True => write!(f, "1"),
            Status::False => write!(f, "0"),
        }
    }
}

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
    let tile: HashMap<(usize, usize), Square> = load_tile(contents)?;
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

fn load_tile(contents: String) -> Result<Tile, String> {
    let mut tile: Tile = HashMap::new();
    let lines = contents.lines().enumerate();
    debug!("Lines: {}", lines.clone().count());
    for (y, line) in lines {
        let chars = line.chars().enumerate();
        debug!("Columns: {}", chars.clone().count());
        for (x, value) in chars {
            tile.insert((x, y), parse_square(&value).unwrap());
        }
    }
    Ok(tile)
}

fn parse_square(square: &char) -> Result<Square, Box<dyn Error>> {
    match square {
        '#' => Ok(Square::True),
        '.' => Ok(Square::False),
        '?' => Ok(Square::Unknown),
        _ => Err(Box::new(ioError::new(
            ErrorKind::InvalidInput,
            format!("Invalid square in the input: {}", square),
        ))),
    }
}

fn validate_tile(tile: &Tile) -> Result<Status, Box<dyn Error>> {
    // We want to figure out a deterministic algorithm so that, for each node in the infinite graph,
    // each node can only observe the 3 × 7 neighborhood of the MIS around themselves and based on that,
    // output a value 1 or 0 so that for each column of the underlying grid, the 1's form a MIS for that particular column.
    //
    // Sadly I already have such an algorithm. It works by outputting 1 when one of the following is true:
    // - The node is a #
    // - The node is the middle node of a vertical streak of _exactly_ three .'s
    // - The node is a part of a vertical streak of strictly more than three .'s and has a # to its right.

    let center = (1, 3); // center of a 3x7 tile
    let to_its_right = (2, 3); // square to its right

    if *tile.get(&center).unwrap() == Square::True {
        Ok(Status::True)
    } else if center_of_three_trues_column(&tile) {
        Ok(Status::True)
    } else if *tile.get(&to_its_right).unwrap() == Square::True && part_of_falsy_streak(&tile) {
        Ok(Status::True)
    } else {
        Ok(Status::False)
    }
}

fn center_of_three_trues_column(tile: &Tile) -> bool {
    // dirty copypaste is easiest here i guess
    *tile.get(&(1, 0)).unwrap() == Square::False
        && *tile.get(&(1, 1)).unwrap() == Square::False
        && *tile.get(&(1, 2)).unwrap() == Square::True
        && *tile.get(&(1, 3)).unwrap() == Square::True
        && *tile.get(&(1, 4)).unwrap() == Square::True
        && *tile.get(&(1, 5)).unwrap() == Square::False
        && *tile.get(&(1, 6)).unwrap() == Square::False
}

fn part_of_falsy_streak(tile: &Tile) -> bool {
    let mut streak = true;
    // we need at least four falses with center included
    // 7 rows means that we have 4 possible falsy streaks
    for shift in 0..=3 {
        streak = true; // assume true for current 4-column
        for row in 0..=3 {
            if *tile.get(&(1, row + shift)).unwrap() == Square::True {
                streak = false; // found a truthy square, breaks our column
                break;
            }
        }
        if streak == true {
            return true; // early return if we went through a valid 4-column
        }
    }

    streak
}

#[test]
fn center_of_three_trues_column_1() {
    let tile = "...
...
...
...
...
...
..."
    .to_string();
    let tile = load_tile(tile).unwrap();
    assert!(center_of_three_trues_column(&tile) == false);
}

#[test]
fn center_of_three_trues_column_2() {
    let tile = "...
...
...
.#.
.#.
.#.
..."
    .to_string();
    let tile = load_tile(tile).unwrap();
    assert!(center_of_three_trues_column(&tile) == false);
}

#[test]
fn center_of_three_trues_column_3() {
    let tile = "...
...
.#.
.#.
.#.
...
..."
    .to_string();
    let tile = load_tile(tile).unwrap();
    assert!(center_of_three_trues_column(&tile) == true);
}

#[test]
fn part_of_falsy_streak_1() {
    let tile = "...
...
...
...
...
...
..."
    .to_string();
    let tile = load_tile(tile).unwrap();
    assert!(part_of_falsy_streak(&tile) == true);
}

#[test]
fn part_of_falsy_streak_2() {
    let tile = ".#.
...
...
...
...
...
..."
    .to_string();
    let tile = load_tile(tile).unwrap();
    assert!(part_of_falsy_streak(&tile) == true);
}

#[test]
fn part_of_falsy_streak_3() {
    let tile = ".#.
.#.
.#.
...
...
...
..."
    .to_string();
    let tile = load_tile(tile).unwrap();
    assert!(part_of_falsy_streak(&tile) == true);
}

#[test]
fn part_of_falsy_streak_4() {
    let tile = ".#.
.#.
.#.
...
...
.#.
..."
    .to_string();
    let tile = load_tile(tile).unwrap();
    assert!(part_of_falsy_streak(&tile) == false);
}

#[test]
fn part_of_falsy_streak_5() {
    let tile = "...
...
...
.#.
...
...
..."
    .to_string();
    let tile = load_tile(tile).unwrap();
    assert!(part_of_falsy_streak(&tile) == false);
}

#[test]
fn validate_tile_1() {
    let tile = "...
...
...
...
...
...
..."
    .to_string();
    let tile = load_tile(tile).unwrap();
    assert!(validate_tile(&tile).unwrap() == Status::False);
}

#[test]
fn validate_tile_2() {
    let tile = "...
...
.#.
.#.
.#.
...
..."
    .to_string();
    let tile = load_tile(tile).unwrap();
    assert!(validate_tile(&tile).unwrap() == Status::True);
}
