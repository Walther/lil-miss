use std::collections::HashMap;
use std::error::Error;
use std::io::{Error as ioError, ErrorKind};

#[derive(std::cmp::PartialEq)]
pub enum Square {
  False,
  True,
  Unknown,
}

pub type Tile = HashMap<(usize, usize), Square>;

#[derive(std::cmp::PartialEq)]
pub enum Status {
  False,
  True,
}
pub fn load_tile(contents: String) -> Result<Tile, String> {
  let mut tile: Tile = HashMap::new();
  let lines = contents.lines().enumerate();
  for (y, line) in lines {
    let chars = line.chars().enumerate();
    for (x, value) in chars {
      tile.insert((x, y), parse_square(&value).unwrap());
    }
  }
  Ok(tile)
}

pub fn parse_square(square: &char) -> Result<Square, Box<dyn Error>> {
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

pub fn validate_tile(tile: &Tile) -> Result<Status, Box<dyn Error>> {
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

pub fn center_of_three_trues_column(tile: &Tile) -> bool {
  // dirty copypaste is easiest here i guess
  *tile.get(&(1, 0)).unwrap() == Square::False
    && *tile.get(&(1, 1)).unwrap() == Square::False
    && *tile.get(&(1, 2)).unwrap() == Square::True
    && *tile.get(&(1, 3)).unwrap() == Square::True
    && *tile.get(&(1, 4)).unwrap() == Square::True
    && *tile.get(&(1, 5)).unwrap() == Square::False
    && *tile.get(&(1, 6)).unwrap() == Square::False
}

pub fn part_of_falsy_streak(tile: &Tile) -> bool {
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
