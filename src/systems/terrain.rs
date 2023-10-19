use super::experiments::gen_planet;
use noise::utils::{ColorGradient, ImageRenderer, NoiseMap};
use noise::{Fbm, Perlin};
use serde_json;
use std::fs::File;

struct TerrainMap {
    map_tiles: Vec<String>,
}

pub fn save_map_as_json(noise_map: NoiseMap) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("example_images/noise.json")?;
    let map: Vec<&f64> = noise_map.iter().collect();
    serde_json::to_writer_pretty(file, &map)?;
    Ok(())
}

pub fn generate_tile_classifications(
    noise_map: &NoiseMap,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("example_images/map_tiles.json")?;
    let final_map_data = classify_noisemap_into_tiles(noise_map);
    serde_json::to_writer_pretty(file, &final_map_data)?;
    Ok(())
}

pub fn classify_point_into_tile(value: f64) -> String {
    if value < 0.0 {
        return "water".into();
    } else if value < 1024.0 / 16384.0 {
        return "grass".into();
    } else if value < 2048.0 / 16384.0 {
        return "dry_land_low".into();
    } else if value < 3072.0 / 16384.0 {
        return "dry_land_high".into();
    } else if value < 4096.0 / 16384.0 {
        return "rocky_low".into();
    } else if value < 5632.0 / 16384.0 {
        return "rocky_high".into();
    } else if value < 6144.0 / 16384.0 {
        return "snowy_low".into();
    } else {
        return "snowy_high".into();
    }
}

pub fn classify_noisemap_into_tiles(noise_map: &NoiseMap) -> Vec<String> {
    let mut classified_map = Vec::new();
    for point in noise_map.iter() {
        classified_map.push(classify_point_into_tile(*point));
    }
    return classified_map;
}

#[rustfmt::skip]
pub fn generate_map(width: usize, height: usize) -> NoiseMap {
    let seed = rand::random::<u32>();
    let mut fbm = Fbm::<Perlin>::new(seed);

    fbm.lacunarity = 2.314453125;

    let noise_map = gen_planet();
    // 192, 14, 14
    
    let color_gradient = ColorGradient::new()
        .clear_gradient()
        // .add_gradient_point(-1.00,              [  0,   0,   0, 255]) // very deep water
        // .add_gradient_point(-256.0 / 16384.0,   [  6,  58, 127, 255]) // deep water
        .add_gradient_point(-1.0 / 16384.0,     [ 14, 112, 192, 255])
        .add_gradient_point(0.0,                [ 70, 120,  60, 255])
        .add_gradient_point(1024.0 / 16384.0,   [110, 140,  75, 255])
        .add_gradient_point(2048.0 / 16384.0,   [160, 140, 111, 255])
        .add_gradient_point(3072.0 / 16384.0,   [184, 163, 141, 255]) // grass_trans_land
        .add_gradient_point(4096.0 / 16384.0,   [128, 128, 128, 255]) // low rocky
        .add_gradient_point(5632.0 / 16384.0,   [152, 161, 163, 255]) // high rocky
        .add_gradient_point(6144.0 / 16384.0,   [250, 250, 250, 255])
        .add_gradient_point(1.0,                [255, 255, 255, 255]);

    let renderer = ImageRenderer::new()
        .set_gradient(color_gradient)
        .render(&noise_map)
        .write_to_file("map_color_gradient.png");

    return noise_map;
}
