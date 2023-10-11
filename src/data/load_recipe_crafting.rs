// This file is auto generated
use crate::resources::recipes::Recipe;
pub fn load_sample_recipes() -> Vec<Recipe> {
    let mut recipes: Vec<Recipe> = Vec::new();
    recipes.push(Recipe::new(vec![("crab", 1), ("anvil", 1)], ("sword", 1)));
    recipes.push(Recipe::new(vec![("anvil", 1), ("sword", 1)], ("sword", 2)));
    recipes.push(Recipe::new(vec![("sword", 1), ("crab", 4)], ("crab", 8)));
    recipes
}
