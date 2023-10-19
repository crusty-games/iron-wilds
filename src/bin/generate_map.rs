use iron_wilds::systems::terrain::{generate_map, generate_tile_classifications, save_map_as_json};

fn main() {
    let map = generate_map(256, 256);
    let _ = generate_tile_classifications(&map);

    let _ = save_map_as_json(map);
}
