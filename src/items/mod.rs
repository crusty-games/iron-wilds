pub mod components;
pub mod plugin;

#[allow(dead_code)]
pub enum ItemKind {}

#[allow(dead_code)]
pub enum ItemComponents {
    None,
    GroundLoot,
    OneWithData { prop: u32 },
}

#[allow(dead_code)]
pub struct ItemProfile {
    pub id: String,
    pub name: String,
    pub components: Vec<ItemComponents>,
}

#[macro_export]
macro_rules! item_has_component {
    ( $item:expr, $pattern:pat, $then:block ) => {
        item_has_component!($item, $pattern, $then, {})
    };
    ( $item:expr, $pattern:pat, $then:block, $else:block ) => {
        #[allow(unused_variables)]
        let opt = $item.components.iter().find(|comp| match comp {
            $pattern => true,
            _ => false,
        });
        if let Some($pattern) = opt $then else $else
    };
}

#[cfg(test)]
mod tests {
    use super::{ItemComponents, ItemProfile};

    #[test]
    fn test_macro() {
        let test_prop = 36;
        let item: ItemProfile = ItemProfile {
            id: "test".into(),
            name: "Test".into(),
            components: vec![
                ItemComponents::OneWithData { prop: test_prop },
                ItemComponents::None,
            ],
        };

        item_has_component!(item, ItemComponents::GroundLoot, { unreachable!() });
        item_has_component!(
            item,
            ItemComponents::OneWithData { prop },
            {
                assert_eq!(prop, &test_prop);
            },
            { unreachable!() }
        );
    }
}

#[allow(dead_code)]
pub fn load_world_items() {
    let mut items: Vec<ItemProfile> = vec![];

    items.push(ItemProfile {
        id: "test".into(),
        name: "Test".into(),
        components: vec![],
    });
}
