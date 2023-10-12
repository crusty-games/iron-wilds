use csv::Reader;
use std::{collections::HashSet, error::Error, fs, process};

type Item = (String, usize);
type Recipe = (String, Item, Vec<Item>);

fn parse_item(str: &str) -> Item {
    let iter = str.split(":").collect::<Vec<&str>>();
    (
        iter.get(0).clone().unwrap().to_string(),
        iter.get(1).clone().unwrap().parse().unwrap(),
    )
}

fn parse_csvs() -> Result<(), Box<dyn Error>> {
    let mut recipes: Vec<Recipe> = Vec::new();
    let mut unique_types: HashSet<String> = HashSet::new();
    let mut reader = Reader::from_path("./data/recipe_crafting.csv")?;
    for result in reader.records() {
        let columns = result?;
        let crafting_type = columns.get(0).unwrap().to_owned();
        let output = parse_item(columns.get(1).unwrap());
        let ingredients = columns.get(2).unwrap().split(",").collect::<Vec<&str>>();
        let ingredients = ingredients
            .iter()
            .map(|s| parse_item(s))
            .collect::<Vec<Item>>();
        unique_types.insert(crafting_type.clone());
        recipes.push((crafting_type, output, ingredients));
    }
    write_code(unique_types, recipes);
    Ok(())
}

fn write_code(unique_types: HashSet<String>, recipes: Vec<Recipe>) {
    let mut code = String::from("use crate::resources::recipes::Recipe;\n");

    for crafting_type in unique_types.iter() {
        code = format!(
            "{}
            #[allow(dead_code)]
            pub fn get_{}_recipes() -> Vec<Recipe> {{
            let mut recipes: Vec<Recipe> = Vec::new();",
            code, crafting_type
        );
        for recipe in recipes.iter() {
            if &recipe.0 == crafting_type {
                code = format!(
                    "{}\nrecipes.push(Recipe::new(vec!{:?}, {:?}));",
                    code, recipe.2, recipe.1,
                );
            }
        }
        code = format!("{}\n    recipes\n}}\n", code);
    }

    fs::write("./src/data/recipes.rs", code).unwrap();
}

fn main() {
    if let Err(err) = parse_csvs() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
