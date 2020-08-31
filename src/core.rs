/// Core Flategy game constructs
use super::util;
use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct UnitID {
    pub index: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct PlayerID {
    pub index: u16,
}

#[derive(Debug, Copy, Clone)]
pub struct GroupID {
    pub index: u8,
}

#[derive(Debug, Copy, Clone)]
pub struct Food {
    pub amount: u32,
}

struct Units {
    position: Vec<Point>, // N
    alive: Vec<bool>,     // N
    owner: Vec<PlayerID>, // N
    group: Vec<GroupID>,  // N
}

#[derive(Debug, Copy, Clone)]
pub enum Tile {
    Empty,
    Wall,
    Food(Food),
    Spawn(PlayerID),
}

pub struct Map {
    pub cells: util::Grid<(usize, usize), Tile>,
}

impl Map {
    pub fn width(&self) -> usize {
        self.cells.shape().0
    }

    pub fn height(&self) -> usize {
        self.cells.shape().1
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
    units: Units,
    spawns: Vec<Point>,                 // P
    commands: Vec<util::VecMap<Point>>, // P.G
}

impl World {
    fn create(spawns: Vec<Point>) -> World {
        let commands = (0..spawns.len()).map(|_| util::VecMap::new()).collect();
        World {
            units: Units {
                position: Vec::new(),
                alive: Vec::new(),
                owner: Vec::new(),
                group: Vec::new(),
            },
            spawns: spawns,
            commands: commands,
        }
    }

    fn spawn(&mut self, player: PlayerID) {
        let spawn = self.spawns[player.index as usize];
        let units = &mut self.units;
        units.position.push(spawn);
        units.alive.push(true);
        units.owner.push(player);
        units.group.push(GroupID { index: 0 });
    }

    fn assign_group(&mut self, unit: UnitID, group: GroupID) {
        self.units.group[unit.index as usize] = group;
    }

    fn set_command(&mut self, player: PlayerID, group: GroupID, command: Option<Point>) {
        self.commands[player.index as usize].insert(group.index as usize, command);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn() {
        let mut world = World::create(vec![Point { x: 10.0, y: 10.0 }, Point { x: 90.0, y: 90.0 }]);
        world.spawn(PlayerID { index: 0 });
        world.assign_group(UnitID { index: 0 }, GroupID { index: 1 });
        world.set_command(
            PlayerID { index: 0 },
            GroupID { index: 1 },
            Some(Point { x: 50.0, y: 50.0 }),
        );
        assert_eq!(1, world.units.position.len());
        assert_eq!(Some(Point { x: 50.0, y: 50.0 }), world.commands[0][1]);
    }
}
