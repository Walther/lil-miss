extern crate lil_miss;
use lil_miss::*;

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
