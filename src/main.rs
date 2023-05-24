use rand::prelude::*;

fn get_random(list: &str) -> &str {
    let mut rng = rand::thread_rng();
    let mut lines = list.lines();
    let index = rng.gen_range(0..lines.clone().count());
    lines.nth(index).unwrap()
}

fn main() {
    let nouns = include_str!("../generated/nouns.txt");
    let adjectives = include_str!("../generated/adjectives.txt");

    let noun = get_random(nouns);
    let adjective = get_random(adjectives);

    println!("{} {}", adjective, noun);
}
