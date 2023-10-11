use csv::Reader;
use std::{error::Error, fs, process};

fn split_item(str: &str) -> (&str, usize) {
    let iter = str.split(":").collect::<Vec<&str>>();
    (
        iter.get(0).clone().unwrap(),
        iter.get(1).clone().unwrap().parse().unwrap(),
    )
}

fn parase_csv() -> Result<(), Box<dyn Error>> {
    let mut code = String::from(
        "// This file is auto generated
        use crate::resources::recipes::Recipe;
        pub fn load_sample_recipes() -> Vec<Recipe> {
        let mut recipes: Vec<Recipe> = Vec::new();",
    );

    let mut rdr = Reader::from_path("./data/recipe_crafting.csv")?;
    for result in rdr.records() {
        let record = result?;
        let ing_split = record.get(0).unwrap().split(",").collect::<Vec<&str>>();
        let ing = ing_split
            .iter()
            .map(|s| split_item(s))
            .collect::<Vec<(&str, usize)>>();
        let out = split_item(record.get(1).unwrap());
        code = format!(
            "{}\nrecipes.push(Recipe::new(vec!{:?}, {:?}));",
            code, ing, out
        );
    }

    code = format!("{}\nrecipes}}", code);

    fs::write("./src/data/load_recipe_crafting.rs", code)?;

    Ok(())
}

fn main() {
    if let Err(err) = parase_csv() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
