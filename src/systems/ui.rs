use bevy::prelude::*;

use crate::components::storage::StorageItem;
use crate::components::ui::{InventoryContainer, InventoryRoot, InventorySlot};
use crate::resources::inventory::Inventory;
use crate::resources::items::{config::AssetConfig, ItemStore};

pub const FONT: &str = "fonts/VT323-Regular.ttf";

pub const INVENTORY_SLOT_SIZE: Val = Val::Px(48.0);
pub const INVENTORY_SLOT_ICON_SIZE: Val = Val::Px(32.0);
pub const INVENTORY_SLOT_GAP: Val = Val::Px(2.0);
pub const INVENTORY_SLOT_BORDER: Val = Val::Px(2.0);
pub const INVENTORY_DEFAULT_BORDER_COLOR: Color = Color::rgba(1.0, 1.0, 1.0, 0.05);
pub const INVENTORY_ACTIVE_BORDER_COLOR: Color = Color::rgba(1.0, 1.0, 1.0, 1.0);

pub fn spawn_inventory_ui(mut commands: Commands, inventory: Res<Inventory>) {
    commands
        .spawn((
            InventoryRoot,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexEnd,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn((
                InventoryContainer,
                NodeBundle {
                    style: Style {
                        height: Val::Auto,
                        width: Val::Auto,
                        ..default()
                    },
                    border_color: Color::RED.into(),
                    ..default()
                },
            ))
            .with_children(|container| {
                for slot_index in inventory.hotbar.range() {
                    container.spawn((
                        InventorySlot { slot_index },
                        ButtonBundle {
                            style: Style {
                                height: INVENTORY_SLOT_SIZE,
                                width: INVENTORY_SLOT_SIZE,
                                border: UiRect::all(INVENTORY_SLOT_BORDER),
                                margin: UiRect::all(INVENTORY_SLOT_GAP),
                                display: Display::Flex,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            border_color: INVENTORY_DEFAULT_BORDER_COLOR.into(),
                            background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                            ..default()
                        },
                    ));
                }
            });
        });
}

pub fn inventory_ui(
    mut commands: Commands,
    mut slots_query: Query<(&InventorySlot, Entity, &mut BorderColor)>,
    inventory: Res<Inventory>,
    item_store: Res<ItemStore>,
    asset_server: Res<AssetServer>,
) {
    if !inventory.is_changed() {
        return;
    }
    for (slot, entity, mut border_color) in slots_query.iter_mut() {
        let mut entity_commands = commands.entity(entity);
        *border_color = BorderColor::from(if slot.slot_index == inventory.hotbar.active_slot {
            INVENTORY_ACTIVE_BORDER_COLOR
        } else {
            INVENTORY_DEFAULT_BORDER_COLOR
        });
        entity_commands.despawn_descendants();
        if let Some(StorageItem {
            item_id,
            stack_count,
        }) = inventory.storage.items.get(&slot.slot_index).unwrap()
        {
            let item = item_store.get(item_id);
            if let Some(AssetConfig { ground_item_path }) = &item.assets {
                entity_commands.with_children(|slot_el| {
                    slot_el.spawn((
                        NodeBundle {
                            style: Style {
                                height: INVENTORY_SLOT_ICON_SIZE,
                                width: INVENTORY_SLOT_ICON_SIZE,
                                ..default()
                            },
                            background_color: Color::WHITE.into(),
                            ..default()
                        },
                        UiImage::new(asset_server.load(ground_item_path)),
                    ));
                    if stack_count > &1 {
                        slot_el.spawn(
                            TextBundle::from_section(
                                stack_count.to_string(),
                                TextStyle {
                                    font: asset_server.load(FONT),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            )
                            .with_style(Style {
                                position_type: PositionType::Absolute,
                                bottom: Val::Px(0.0),
                                right: Val::Px(5.0),
                                ..default()
                            }),
                        );
                    }
                });
            }
        }
    }
}

pub fn click_inventory_slot(
    slots_query: Query<(&InventorySlot, &Interaction)>,
    mut inventory: ResMut<Inventory>,
) {
    for (slot, interaction) in slots_query.iter() {
        if matches!(interaction, Interaction::Pressed) {
            inventory.hotbar.active_slot = slot.slot_index;
        }
    }
}
