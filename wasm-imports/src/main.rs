extern crate itertools;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::collections::HashMap;
use Direction::*;
use itertools::Itertools;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Serialize, Deserialize, Debug)]
struct Example {
    favorite_animal: String,
    favorite_direction: Direction,
}

fn main() {
    let mut users_facing = HashMap::new();
    users_facing.insert("Alice", North);
    users_facing.insert("Arul", South);
    users_facing.insert("Clarice", East);

    let users_not_facing_north = users_facing
        .iter()
        .filter(|&(_, d)| *d != North)
        .collect::<HashMap<_, _>>();
    println!("{:?}", users_not_facing_north);

    let directions = vec![North, South, West, East, West, North, South];
    let unique_directions = directions.iter().unique().collect::<Vec<_>>();
    println!("{:?}", unique_directions);

    let data = r#"{"favorite_animal":"Bear", "favorite_direction":"North"}"#;
    let parsed: Example = serde_json::from_str(data).unwrap();
    println!("{:?}", parsed);
}
