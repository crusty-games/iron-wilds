#[macro_export]
macro_rules! bevy_component {
    ($name:ident) => {
        #[derive(Component)]
        pub struct $name;
    };
}
