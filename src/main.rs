mod fsio;

extern crate serde_json;
use serde::Deserialize;

// toggle verbose output for debug
const DEBUG: i8 = 0; // for verbose set to = 1

#[derive(Deserialize, Debug, Clone)]
struct Item {
    name: String,
    baskets: Vec<usize>,
}
#[derive(Deserialize, Debug, Clone)]
struct Basket {
    items: Vec<Item>,
}

fn main() {
    println!("{}", fsio::parse_json("src/basket.json"));
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_basket_content() -> () {
        let good = "60 apples in 3 baskets\n45 bananas in 4 baskets";
        let this: String = fsio::parse_json("src/basket.json");
        assert_eq!(this, good);
    }

    #[test]
    fn test_basket_error() -> () {
        let good = "60 apples in 3 baskets\n41 bananas in 4 baskets";
        let this: String = fsio::parse_json("src/basket.json");
        assert_ne!(this, good);
    }

}
