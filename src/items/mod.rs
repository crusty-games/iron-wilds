pub mod components;
pub mod game_items;
pub mod plugin;

#[allow(dead_code)]
pub enum GameItemComponent {
    Placable,
    GroundLoot,
    Stackable { max_stack: usize },
}

#[allow(dead_code)]
pub struct GameItem {
    pub id: String,
    pub name: String,
    pub components: Vec<GameItemComponent>,
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

#[test]
fn test_macro() {
    let test_value = 36;
    let item: GameItem = GameItem {
        id: "test".into(),
        name: "Test".into(),
        components: vec![
            GameItemComponent::Stackable {
                max_stack: test_value,
            },
            GameItemComponent::Placable,
        ],
    };

    item_has_component!(item, GameItemComponent::GroundLoot, { unreachable!() });
    item_has_component!(
        item,
        GameItemComponent::Stackable { max_stack },
        {
            assert_eq!(max_stack, &test_value);
        },
        { unreachable!() }
    );
}
