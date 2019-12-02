# Advent of Code 2019 Rust Solutions

## Status

sunday | monday | tuesday | wednesday | thursday | friday | saturday
------ | ------ | ------- | --------- | -------- | ------ | --------
1 🌟🌟 | 2 🌟🌟 | 3       | 4         | 5        | 6      | 7
8      | 9      | 10      | 11        | 12       | 13     | 14
15     | 16     | 17      | 18        | 19       | 20     | 21
22     | 23     | 24      | 25        | 26       | 27     | 28
29     | 30     | 31

## Usage

To run a given day do something like this:

```sh
cargo run -- one ./one/input
cargo run -- two 1,0,0,0,99
```

This will run the last version of the puzzle, so if it's a two part puzzle this
will run the second part (since it usually builds upon the first part and I am
too lazy to make a sub-command for both parts).

To run automated tests for all solutions:

```sh
cargo test
```
