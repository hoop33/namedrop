use std::error::Error;
use rand::prelude::*;

#[derive(Debug)]
pub struct NameDrop {
    nouns: &'static str,
    adjectives: &'static str,
}

impl NameDrop {
    pub fn new() -> Self {
        Self {
            nouns: include_str!("../generated/nouns.txt"),
            adjectives: include_str!("../generated/adjectives.txt"),
        }
    }

    pub fn run(&self, count: u8) -> Result<(), Box<dyn Error>> {
        for _ in 0..count {
            println!("{}", self.get_random_name());
        }
        Ok(())
    }

    fn get_random_name(&self) -> String {
        format!("{} {}", get_random(self.adjectives), get_random(self.nouns))
    }
}

fn get_random(list: &str) -> &str {
    let mut rng = rand::thread_rng();
    let mut lines = list.lines();
    let count = lines.clone().count();
    match count {
        0 => "",
        _ => {
            let index = rng.gen_range(0..count);
            lines.nth(index).unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_gets_random_item_from_list() {
        let list = "one\ntwo\nthree\nfour\nfive\nsix\nseven\neight\nnine\nten";
        let result = get_random(list);
        assert!(list.contains(result));
    }

    #[test]
    fn test_get_random_gets_blank_when_no_list() {
        let list = "";
        let result = get_random(list);
        assert_eq!(result, "");
    }

    #[test]
    fn test_namedrop_get_random_name_returns_random_name_with_space() {
        let name_drop = NameDrop::new();
        let result = name_drop.get_random_name();
        assert!(result.contains(" "));
    }

    #[test]
    fn test_namedrop_get_random_name_returns_random_name_with_at_least_two_words() {
        let name_drop = NameDrop::new();
        let result = name_drop.get_random_name();
        assert!(result.split(" ").count() >= 2);
    }
}

