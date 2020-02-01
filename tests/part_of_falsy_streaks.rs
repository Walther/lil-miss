extern crate lil_miss;
use lil_miss::*;

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
fn part_of_falsy_streak_6() {
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
