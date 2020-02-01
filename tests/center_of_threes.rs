extern crate lil_miss;
use lil_miss::*;

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
