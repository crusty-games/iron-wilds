use crate::resources::recipes::Recipe;

#[allow(dead_code)]
pub fn get_crafting_basic_recipes() -> Vec<Recipe> {
    let mut recipes: Vec<Recipe> = Vec::new();
    recipes.push(Recipe::new(vec![("crab", 1), ("sword", 1)], ("crab", 2)));
    recipes
}

#[allow(dead_code)]
pub fn get_cooking_basic_recipes() -> Vec<Recipe> {
    let mut recipes: Vec<Recipe> = Vec::new();
    recipes.push(Recipe::new(vec![("crab", 1), ("anvil", 2)], ("crab", 2)));
    recipes
}

#[allow(dead_code)]
pub fn get_crafting_workbench_recipes() -> Vec<Recipe> {
    let mut recipes: Vec<Recipe> = Vec::new();
    recipes.push(Recipe::new(vec![("crab", 1), ("anvil", 1)], ("crab", 2)));
    recipes
}

#[allow(dead_code)]
pub fn get_cooking_kitchen_recipes() -> Vec<Recipe> {
    let mut recipes: Vec<Recipe> = Vec::new();
    recipes.push(Recipe::new(vec![("crab", 1), ("anvil", 3)], ("crab", 2)));
    recipes
}
