/// Core Flategy game constructs
use super::util;
use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UnitID(pub u32);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PlayerID(pub u16);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GroupID(pub u8);

pub struct Units {
    pub position: Vec<Point>, // U
    pub alive: Vec<bool>,     // U
    pub owner: Vec<PlayerID>, // U
    pub group: Vec<GroupID>,  // U
}

#[derive(Debug, Copy, Clone)]
pub enum Tile {
    Empty,
    Wall,
    Food { max_workers: u8 },
    Spawn(PlayerID),
}

type Coordinate = (usize, usize);

#[derive(Clone)]
pub struct Map {
    pub tiles: util::Grid<Coordinate, Tile>,
}

impl Map {
    pub fn empty_2p(size: usize) -> Self {
        let mut grid = util::Grid::new((size, size), &Tile::Empty);
        grid[(0, 0)] = Tile::Spawn(PlayerID(0));
        grid[(size - 1, size - 1)] = Tile::Spawn(PlayerID(1));
        Map { tiles: grid }
    }

    pub fn choke_2p(size: usize) -> Self {
        assert!(
            size % 2 == 1,
            "choke_2p map size must be odd, is {:?}",
            size
        );
        let mut map = Self::empty_2p(size);
        let index = size / 2;
        let length = index - 1;
        for i in 0..length {
            map.tiles[(index, i)] = Tile::Wall;
            map.tiles[(index, size - 1 - i)] = Tile::Wall;
            map.tiles[(i, index)] = Tile::Wall;
            map.tiles[(size - 1 - i, index)] = Tile::Wall;
        }
        map.tiles[(0, size - 1)] = Tile::Food { max_workers: 4 };
        map.tiles[(size - 1, 0)] = Tile::Food { max_workers: 4 };
        map
    }

    pub fn center(coordinate: Coordinate) -> Point {
        Point {
            x: coordinate.0 as f32 + 0.5,
            y: coordinate.1 as f32 + 0.5,
        }
    }

    pub fn width(&self) -> usize {
        self.tiles.shape().0
    }

    pub fn height(&self) -> usize {
        self.tiles.shape().1
    }

    pub fn n_players(&self) -> usize {
        self.tiles
            .data()
            .iter()
            .filter(|x| match x {
                Tile::Spawn(_) => true,
                _ => false,
            })
            .count()
    }

    pub fn find_spawn(&self, player: &PlayerID) -> Point {
        let found = self.tiles.data().iter().enumerate().find(|(_, x)| match x {
            Tile::Spawn(p) if p == player => true,
            _ => false,
        });

        if let Some((idx, _)) = found {
            Self::center(self.tiles.from_offset(idx))
        } else {
            panic!("Player {:?} was not found in map", player);
        }
    }

    pub fn dump(&self) -> String {
        let body: String = self
            .tiles
            .data()
            .chunks(self.width())
            .map(|chunk| {
                let row: String = chunk
                    .iter()
                    .map(|tile| match tile {
                        Tile::Empty => "  ",
                        Tile::Wall => "##",
                        Tile::Food { max_workers: _ } => "::",
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

pub struct World {
    pub map: Map,
    pub units: Units,                       // U
    pub commands: Vec<util::VecMap<Point>>, // P.G
}

impl World {
    pub fn create(map: Map) -> World {
        let commands = (0..map.n_players()).map(|_| util::VecMap::new()).collect();
        World {
            map: map,
            units: Units {
                position: Vec::new(),
                alive: Vec::new(),
                owner: Vec::new(),
                group: Vec::new(),
            },
            commands: commands,
        }
    }

    pub fn spawn(&mut self, player: PlayerID) {
        let spawn = self.map.find_spawn(&player);
        let units = &mut self.units;
        units.position.push(spawn);
        units.alive.push(true);
        units.owner.push(player);
        units.group.push(GroupID(0));
    }

    pub fn assign_group(&mut self, unit: UnitID, group: GroupID) {
        self.units.group[unit.0 as usize] = group;
    }

    pub fn set_command(&mut self, player: PlayerID, group: GroupID, command: Option<Point>) {
        self.commands[player.0 as usize].insert(group.0 as usize, command);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let map = Map::empty_2p(5);
        assert_eq!(5, map.width());
        assert_eq!(5, map.height());
        assert_eq!(2, map.n_players());
        assert_eq!(Point { x: 0.5, y: 0.5 }, map.find_spawn(&PlayerID(0)));
        assert_eq!(Point { x: 4.5, y: 4.5 }, map.find_spawn(&PlayerID(1)));
    }

    #[test]
    fn test_spawn() {
        let mut world = World::create(Map::choke_2p(7));
        world.spawn(PlayerID(0));
        world.assign_group(UnitID(0), GroupID(1));
        world.set_command(PlayerID(0), GroupID(1), Some(Point { x: 2.5, y: 0.5 }));
        assert_eq!(1, world.units.position.len());
        assert_eq!(Some(Point { x: 2.5, y: 0.5 }), world.commands[0][1]);
    }
}
