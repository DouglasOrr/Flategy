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
pub struct GroupID(pub u16);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Food {
    pub amount: u32,
}

struct Units {
    position: Vec<Point>, // U
    alive: Vec<bool>,     // U
    owner: Vec<PlayerID>, // U
    group: Vec<GroupID>,  // U
}

#[derive(Debug, Copy, Clone)]
pub enum Tile {
    Empty,
    Wall,
    Food(Food),
    Spawn(PlayerID),
}

type Coordinate = (usize, usize);

#[derive(Clone)]
pub struct Map {
    pub cells: util::Grid<Coordinate, Tile>,
}

impl Map {
    pub fn empty_2p(size: usize) -> Self {
        let mut grid = util::Grid::new((size, size), &Tile::Empty);
        grid[(0, 0)] = Tile::Spawn(PlayerID(0));
        grid[(size - 1, size - 1)] = Tile::Spawn(PlayerID(1));
        Map { cells: grid }
    }

    pub fn center(coordinate: Coordinate) -> Point {
        Point {
            x: coordinate.0 as f32 + 0.5,
            y: coordinate.1 as f32 + 0.5,
        }
    }

    pub fn width(&self) -> usize {
        self.cells.shape().0
    }

    pub fn height(&self) -> usize {
        self.cells.shape().1
    }

    pub fn n_players(&self) -> usize {
        self.cells
            .data()
            .iter()
            .filter(|x| match x {
                Tile::Spawn(_) => true,
                _ => false,
            })
            .count()
    }

    pub fn find_spawn(&self, player: &PlayerID) -> Point {
        let found = self.cells.data().iter().enumerate().find(|(_, x)| match x {
            Tile::Spawn(p) if p == player => true,
            _ => false,
        });

        if let Some((idx, _)) = found {
            Self::center(self.cells.from_offset(idx))
        } else {
            panic!("Player {:?} was not found in map", player);
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

struct World {
    map: Map,
    units: Units,                       // U
    commands: Vec<util::VecMap<Point>>, // P.G
}

impl World {
    fn create(map: Map) -> World {
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

    fn spawn(&mut self, player: PlayerID) {
        let spawn = self.map.find_spawn(&player);
        let units = &mut self.units;
        units.position.push(spawn);
        units.alive.push(true);
        units.owner.push(player);
        units.group.push(GroupID(0));
    }

    fn assign_group(&mut self, unit: UnitID, group: GroupID) {
        self.units.group[unit.0 as usize] = group;
    }

    fn set_command(&mut self, player: PlayerID, group: GroupID, command: Option<Point>) {
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
        assert_eq!(Point {x: 0.5, y: 0.5}, map.find_spawn(&PlayerID(0)));
        assert_eq!(Point {x: 4.5, y: 4.5}, map.find_spawn(&PlayerID(1)));
    }

    #[test]
    fn test_spawn() {
        let mut world = World::create(Map::empty_2p(5));
        world.spawn(PlayerID(0));
        world.assign_group(UnitID(0), GroupID(1));
        world.set_command(
            PlayerID(0),
            GroupID(1),
            Some(Point { x: 2.5, y: 0.5 }),
        );
        assert_eq!(1, world.units.position.len());
        assert_eq!(Some(Point { x: 2.5, y: 0.5 }), world.commands[0][1]);
    }
}
