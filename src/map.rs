/// Representation and generation of maps (fixed terrain)
use super::{core, util};
use itertools::Itertools;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

#[derive(Debug, Copy, Clone)]
enum Tile {
    Empty,
    Wall,
    Food(core::Food),
    Spawn(core::PlayerID),
}

pub struct Map {
    cells: util::Grid<(usize, usize), Tile>,
}

impl Map {
    pub fn width(&self) -> usize {
        self.cells.shape().0
    }

    pub fn height(&self) -> usize {
        self.cells.shape().1
    }

    pub fn generate(seed: u64) -> Map {
        let mut rng = StdRng::seed_from_u64(seed);

        // Overall map
        let mut map = Map {
            cells: util::Grid::new((rng.gen_range(10, 15), rng.gen_range(10, 15)), &Tile::Empty),
        };

        // Spawn points
        let spawn_0 = (
            rng.gen_range(0, map.width() / 3),
            rng.gen_range(0, map.height() / 3),
        );
        map.cells[spawn_0] = Tile::Spawn(core::PlayerID { index: 0 });
        let spawn_1 = (map.width() - 1 - spawn_0.0, map.height() - 1 - spawn_0.1);
        map.cells[spawn_1] = Tile::Spawn(core::PlayerID { index: 1 });

        // Food
        map.generate_tiles(
            rng.gen_range(map.cells.len() / 20, map.cells.len() / 10),
            || Tile::Food(core::Food { amount: 100 }),
            &mut rng,
        );

        // Walls
        map.generate_tiles(
            rng.gen_range(map.cells.len() / 10, map.cells.len() / 4),
            || Tile::Wall,
            &mut rng,
        );

        map
    }

    fn generate_tiles<F>(&mut self, max: usize, generate: F, rng: &mut StdRng)
    where
        F: Fn() -> Tile,
    {
        for _ in 0..max {
            let index = (
                rng.gen_range(0, self.width()),
                rng.gen_range(0, self.height()),
            );
            match self.cells[index] {
                Tile::Empty => self.cells[index] = generate(),
                _ => (),
            }
        }
    }

    pub fn dump(&self) -> String {
        let body: String = self
            .cells
            .data()
            .chunks(self.width())
            .map(|chunk| {
                let row: String = chunk
                    .iter()
                    .map(|cell| match cell {
                        Tile::Empty => "  ",
                        Tile::Wall => "##",
                        Tile::Food(_) => "::",
                        Tile::Spawn(_) => "()",
                    })
                    .collect();
                format!("| {} |", row)
            })
            .intersperse("\n".to_string())
            .collect();
        let hline = "-".repeat(2 * self.width());
        format!("+-{}-+\n{}\n+-{}-+", hline, body, hline)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let map = Map::generate(75);
        assert!(map.width() * map.height() == map.cells.len());

        let dump = map.dump();
        println!("{}", dump);
        assert!(dump.matches("()").count() == 2);
    }
}
