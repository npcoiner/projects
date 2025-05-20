//! Top-down chunked terrain viewer with WASD camera control
#![allow(non_snake_case)]

use crossterm::{
    event::{read, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use noise::{NoiseFn, Perlin};
use rand::Rng;
use std::{collections::HashMap, io::{stdout, Write}};

const WIDTH: usize = 184;
const HEIGHT: usize = 44;
const CHUNK_SIZE: usize = 64;

#[derive(Clone, Copy, Debug)]
enum TileType {
    Grass,
    Water,
    Sand,
    Rock,
}

#[derive(Clone, Copy, Debug)]
enum EntityType {
    Tree,
    Stone,
    Shell,
}

fn tile_colors(tile: TileType) -> (u8, u8, u8) {
    match tile {
        TileType::Grass => (0, 180, 0),
        TileType::Water => (0, 90, 200),
        TileType::Sand => (230, 210, 150),
        TileType::Rock => (90, 90, 90),
    }
}

fn entity_color(entity: EntityType) -> (u8, u8, u8) {
    match entity {
        EntityType::Tree => (20, 120, 20),
        EntityType::Stone => (180, 180, 180),
        EntityType::Shell => (255, 245, 220),
    }
}

#[derive(Clone, Copy)]
struct Pixel {
    color: (u8, u8, u8),
}

struct PixelBuffer {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Pixel>>,
}

impl PixelBuffer {
    fn new(width: usize, height: usize) -> Self {
        let default = Pixel { color: (0, 0, 0) };
        let pixels = vec![vec![default; width]; height];
        PixelBuffer { width, height, pixels }
    }

    fn set(&mut self, x: usize, y: usize, color: (u8, u8, u8)) {
        if x < self.width && y < self.height {
            self.pixels[y][x].color = color;
        }
    }

    fn render(&self) -> String {
        let mut out = String::new();
        for y in (0..self.height).step_by(2) {
            for x in 0..self.width {
                let top = self.pixels[y][x].color;
                let bottom = if y + 1 < self.height {
                    self.pixels[y + 1][x].color
                } else {
                    (0, 0, 0)
                };
                out.push_str(&format!(
                    "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m▀",
                    top.0, top.1, top.2,
                    bottom.0, bottom.1, bottom.2
                ));
            }
            out.push_str("\r\n");
        }
        out
    }
}

type Chunk = Vec<Vec<TileType>>;
type EntityList = Vec<(usize, usize, EntityType)>;
type ChunkCoord = (i32, i32);

struct World {
    seed: u32,
    chunks: HashMap<ChunkCoord, Chunk>,
    entities: HashMap<ChunkCoord, EntityList>,
}

impl World {
    fn new(seed: u32) -> Self {
        World {
            seed,
            chunks: HashMap::new(),
            entities: HashMap::new(),
        }
    }

    fn get_tile(&mut self, world_x: i32, world_y: i32) -> TileType {
        let chunk_x = world_x.div_euclid(CHUNK_SIZE as i32);
        let chunk_y = world_y.div_euclid(CHUNK_SIZE as i32);
        let local_x = world_x.rem_euclid(CHUNK_SIZE as i32) as usize;
        let local_y = world_y.rem_euclid(CHUNK_SIZE as i32) as usize;

        let chunk_coord = (chunk_x, chunk_y);
        if !self.chunks.contains_key(&chunk_coord) {
            let (tiles, ents) = generate_chunk_with_entities(self.seed, chunk_x, chunk_y);
            self.chunks.insert(chunk_coord, tiles);
            self.entities.insert(chunk_coord, ents);
        }

        self.chunks.get(&chunk_coord).unwrap()[local_y][local_x]
    }

    fn get_entities_in_view(&self, camera_x: i32, camera_y: i32) -> Vec<(i32, i32, EntityType)> {
        let mut results = vec![];
        for ((cx, cy), ents) in &self.entities {
            for &(lx, ly, entity) in ents {
                let wx = cx * CHUNK_SIZE as i32 + lx as i32;
                let wy = cy * CHUNK_SIZE as i32 + ly as i32;
                if wx >= camera_x && wx < camera_x + WIDTH as i32 && wy >= camera_y && wy < camera_y + (HEIGHT * 2) as i32 {
                    results.push((wx, wy, entity));
                }
            }
        }
        results
    }
}

fn generate_chunk_with_entities(seed: u32, chunk_x: i32, chunk_y: i32) -> (Chunk, EntityList) {
    let perlin = Perlin::new(seed);
    let scale = 0.01;
    let mut chunk = vec![vec![TileType::Grass; CHUNK_SIZE]; CHUNK_SIZE];
    let mut entities = vec![];
    let mut rng = rand::rng();

    for y in 0..CHUNK_SIZE {
        for x in 0..CHUNK_SIZE {
            let world_x = (chunk_x * CHUNK_SIZE as i32 + x as i32) as f64;
            let world_y = (chunk_y * CHUNK_SIZE as i32 + y as i32) as f64;
            let val = perlin.get([world_x * scale, world_y * scale]);

            let tile = match val {
                v if v < -0.3 => TileType::Water,
                v if v < 0.0 => TileType::Sand,
                v if v < 0.5 => TileType::Grass,
                _ => TileType::Rock,
            };

            if rng.random::<f32>() < 0.002 {
                if let Some(entity) = match tile {
                    TileType::Grass => Some(EntityType::Tree),
                    TileType::Rock => Some(EntityType::Stone),
                    TileType::Sand => Some(EntityType::Shell),
                    _ => None,
                } {
                    entities.push((x, y, entity));
                }
            }

            chunk[y][x] = tile;
        }
    }

    (chunk, entities)
}

fn draw_frame(world: &mut World, camera_x: i32, camera_y: i32) {
    let mut buffer = PixelBuffer::new(WIDTH, HEIGHT * 2);

    for y in 0..(HEIGHT * 2) {
        for x in 0..WIDTH {
            let wx = camera_x + x as i32;
            let wy = camera_y + y as i32;
            buffer.set(x, y, tile_colors(world.get_tile(wx, wy)));
        }
    }

    for (wx, wy, entity) in world.get_entities_in_view(camera_x, camera_y) {
        let sx = (wx - camera_x) as usize;
        let sy = (wy - camera_y) as usize;
        if sx < WIDTH && sy < HEIGHT * 2 {
            buffer.set(sx, sy, entity_color(entity));
        }
    }

    let px = WIDTH / 2 - 1;
    let py = HEIGHT - 1;
    for dy in 0..4 {
        for dx in 0..3 {
            buffer.set(px + dx, py + dy, (255, 255, 255));
        }
    }

    print!("\x1b[1;1H{}", buffer.render());
    print!("\x1b[{};1H", HEIGHT + 1);
    stdout().flush().unwrap();
}

fn init_terminal() {
    enable_raw_mode().unwrap();
    print!("\x1B[2J\x1B[3J\x1B[H");
    stdout().flush().unwrap();
}

fn shutdown_terminal() {
    println!("\nExiting cleanly...");
    disable_raw_mode().unwrap();
}

fn main() {
    init_terminal();
    let mut rng = rand::rng();
    let seed = rng.random();
    let mut world = World::new(seed);

    let mut camera_x: i32 = 0;
    let mut camera_y: i32 = 0;

    draw_frame(&mut world, camera_x, camera_y);

    loop {
        if let Event::Key(KeyEvent { code, .. }) = read().unwrap() {
            match code {
                KeyCode::Char('q') => { shutdown_terminal(); return; },
                KeyCode::Char('w') => camera_y -= 1,
                KeyCode::Char('s') => camera_y += 1,
                KeyCode::Char('a') => camera_x -= 1,
                KeyCode::Char('d') => camera_x += 1,
                _ => {}
            }
            draw_frame(&mut world, camera_x, camera_y);
        }
    }
}
