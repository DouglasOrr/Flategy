/// Representation and generation of maps (fixed terrain)
use super::core;
use itertools::Itertools;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::ops::{Index, IndexMut};

#[derive(Debug, Copy, Clone)]
enum Tile {
    Empty,
    Wall,
    Food(core::Food),
    Spawn(core::PlayerID),
}

struct Map {
    width: u16,
    height: u16,
    cells: Vec<Tile>,
}

impl Index<(u16, u16)> for Map {
    type Output = Tile;

    fn index(&self, index: (u16, u16)) -> &Self::Output {
        let index = self.get_index(index.0, index.1);
        &self.cells[index]
    }
}

impl IndexMut<(u16, u16)> for Map {
    fn index_mut(&mut self, index: (u16, u16)) -> &mut Self::Output {
        let index = self.get_index(index.0, index.1);
        &mut self.cells[index]
    }
}

impl Map {
    fn get_index(&self, x: u16, y: u16) -> usize {
        (y as usize) * (self.width as usize) + (x as usize)
    }

    fn generate(seed: u64) -> Map {
        let mut rng = StdRng::seed_from_u64(seed);

        // Overall map
        let width = rng.gen_range(10, 15);
        let height = rng.gen_range(10, 15);
        let count = (height as usize) * (width as usize);
        let mut map = Map {
            width: width,
            height: height,
            cells: [Tile::Empty].repeat(count),
        };

        // Spawn points
        let spawn_x = rng.gen_range(0, width / 3);
        let spawn_y = rng.gen_range(0, height / 3);
        map[(spawn_x, spawn_y)] = Tile::Spawn(core::PlayerID { index: 0 });
        map[(width - 1 - spawn_x, height - 1 - spawn_y)] = Tile::Spawn(core::PlayerID { index: 1 });

        // Food
        map.generate_tiles(
            rng.gen_range(count / 20, count / 10),
            || Tile::Food(core::Food { amount: 100 }),
            &mut rng,
        );

        // Walls
        map.generate_tiles(
            rng.gen_range(count / 10, count / 4),
            || Tile::Wall,
            &mut rng,
        );

        map
    }

    fn generate_tiles<F>(&mut self, max: usize, generate: F, rng: &mut StdRng)
    where
        F: Fn() -> Tile,
    {
        let count = self.cells.len();
        for _ in 0..max {
            let index = rng.gen_range(0, count);
            match self.cells[index] {
                Tile::Empty => self.cells[index] = generate(),
                _ => (),
            }
        }
    }

    fn dump(&self) -> String {
        self.cells
            .chunks(self.width as usize)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|cell| match cell {
                        Tile::Empty => "  ",
                        Tile::Wall => "##",
                        Tile::Food(_) => "::",
                        Tile::Spawn(_) => "()",
                    })
                    .collect()
            })
            .intersperse("\n".to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let map = Map::generate(75);
        assert!((map.width * map.height) as usize == map.cells.len());

        let dump = map.dump();
        println!("{}", dump);
        assert!(dump.matches("()").count() == 2);
    }
}
