use std::{thread::sleep, time::Duration};

use wasm_game_of_life::universe::Universe;

fn main() {
    let mut universe: Universe = include_str!("example-universe.txt").try_into().unwrap();
    loop {
        println!("{universe}");

        universe = universe.step();
        sleep(Duration::from_millis(100));
    }
}
