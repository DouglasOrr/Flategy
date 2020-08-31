/// Representation and generation of maps (fixed terrain)
use super::{core, util};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

impl core::Map {
    pub fn generate_2p(seed: u64) -> core::Map {
        let mut rng = StdRng::seed_from_u64(seed);

        // Overall map
        let mut map = core::Map {
            tiles: util::Grid::new(
                (rng.gen_range(10, 15), rng.gen_range(10, 15)),
                &core::Tile::Empty,
            ),
        };

        // Spawn points
        let spawn_0 = (
            rng.gen_range(0, map.width() / 3),
            rng.gen_range(0, map.height() / 3),
        );
        map.tiles[spawn_0] = core::Tile::Spawn(core::PlayerID(0));
        let spawn_1 = (map.width() - 1 - spawn_0.0, map.height() - 1 - spawn_0.1);
        map.tiles[spawn_1] = core::Tile::Spawn(core::PlayerID(1));

        // Food
        map.generate_tiles(
            rng.gen_range(map.tiles.len() / 20, map.tiles.len() / 10),
            || core::Tile::Food { max_workers: 4 },
            &mut rng,
        );

        // Walls
        map.generate_tiles(
            rng.gen_range(map.tiles.len() / 10, map.tiles.len() / 4),
            || core::Tile::Wall,
            &mut rng,
        );

        map
    }

    fn generate_tiles<F>(&mut self, max: usize, generate: F, rng: &mut StdRng)
    where
        F: Fn() -> core::Tile,
    {
        for _ in 0..max {
            let index = (
                rng.gen_range(0, self.width()),
                rng.gen_range(0, self.height()),
            );
            match self.tiles[index] {
                core::Tile::Empty => self.tiles[index] = generate(),
                _ => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_2p() {
        let map = core::Map::generate_2p(75);
        assert!(map.width() * map.height() == map.tiles.len());

        assert_eq!(2, map.n_players());

        let dump = map.dump();
        println!("{}", dump);
        assert!(dump.matches("()").count() == 2);
    }
}
