mod store;

#[derive(Clone)]
pub struct PropFood {
    health_increase: f32,
}

#[derive(Clone)]
pub struct PropMaterial;

#[derive(Clone)]
pub struct GameItem {
    pub id: String,
    pub name: String,
    pub prop_food: Option<PropFood>,
    pub prop_material: Option<PropMaterial>,
}

impl Default for GameItem {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            prop_food: Default::default(),
            prop_material: Default::default(),
        }
    }
}
