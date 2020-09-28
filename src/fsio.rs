use std::fs::File;
use std::io::{self, Read};

use crate::Basket;
use crate::DEBUG;

pub fn read(file: &str) -> Result<String, io::Error> {
    let fh = File::open(file);
    let mut f = match fh {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

pub fn parse_json(file: &str) -> String {
    let fh = match read(file) {
        // gently massage the data to fit our struct
        //Ok(basket_file) => { let mut cat = "{\"items\":".to_owned(); cat.push_str(&basket_file); cat.push_str("}"); cat },
        Ok(basket_file) => format!("{}\"items\":{}{}", "{", &basket_file, "}"),
        _ => panic!("Unable to read {}", file),
    };
    if DEBUG >= 1 {
        println!("{:?}", fh);
    }
    let basket = match serde_json::from_str::<Basket>(&fh) {
        Ok(json) => json,
        Err(e) => {
            panic!("Can't parse json {:?}", e);
        }
    };
    if DEBUG >= 1 {
        println!("{:?}", &basket);
    }

    /* 
       Go through each item and tally up the baskets and report(basket) -> 
       (tally, basket.items.[item].name, basket.items.[item].baskets.len()){} 
    */
    let mut text_from_json: String = "".to_string();
    for (i, item) in (basket.items).iter().enumerate() {
        text_from_json = format!(
            "{}{} {} in {} baskets",
            text_from_json,
            item.baskets.iter().sum::<usize>(),
            item.name,
            item.baskets.len()
        );
        // add new line to all but the last entry
        match i + 1 < basket.items.len() {
            true => text_from_json = format!("{}{}", text_from_json, '\n'),
            _ => {}
        };
    }
    text_from_json
}
