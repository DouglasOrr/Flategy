/// Rendering game state (to JSON) for viewing in-browser
///
/// Note that we choose to keep the rendering type definitions separate,
/// to avoid coupling & make the data transformation "obvious".
///
use super::core;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Wall(usize, usize);

#[derive(Debug, Serialize, Clone)]
pub struct Food(usize, usize, u8);

#[derive(Debug, Serialize, Clone)]
pub struct Spawn(usize, usize, u16);

#[derive(Debug, Serialize, Clone)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub walls: Vec<Wall>,
    pub foods: Vec<Food>,
    pub spawns: Vec<Spawn>,
}

#[derive(Debug, Serialize)]
pub struct Units {
    x: Vec<f32>,
    y: Vec<f32>,
    owner: Vec<u16>,
}

pub fn render_map(map: &core::Map) -> Map {
    let mut result = Map {
        width: map.width(),
        height: map.height(),
        walls: Vec::new(),
        foods: Vec::new(),
        spawns: Vec::new(),
    };
    for idx in 0..map.tiles.len() {
        let (x, y) = map.tiles.from_offset(idx);
        match map.tiles.data()[idx] {
            core::Tile::Empty => (),
            core::Tile::Wall => result.walls.push(Wall(x, y)),
            core::Tile::Food { max_workers } => result.foods.push(Food(x, y, max_workers)),
            core::Tile::Spawn(id) => result.spawns.push(Spawn(x, y, id.0)),
        }
    }
    result
}

pub fn render_units_to(src: &core::Units, dest: &mut Units) {
    dest.x.clear();
    dest.y.clear();
    dest.owner.clear();
    for idx in 0..src.position.len() {
        if src.alive[idx] {
            dest.x.push(src.position[idx].x);
            dest.y.push(src.position[idx].y);
            dest.owner.push(src.owner[idx].0);
        }
    }
}

pub fn render_units(units: &core::Units) -> Units {
    let count = units.alive.iter().map(|b| *b as usize).sum();
    let mut result = Units {
        x: Vec::with_capacity(count),
        y: Vec::with_capacity(count),
        owner: Vec::with_capacity(count),
    };
    render_units_to(units, &mut result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_map() {
        let map = core::Map::choke_2p(7);
        let rmap = render_map(&map);
        assert_eq!(7, rmap.width);
        assert_eq!(7, rmap.height);
        assert_eq!(2 * 4, rmap.walls.len());
        assert_eq!(2, rmap.foods.len());
        assert_eq!(2, rmap.spawns.len());
    }

    #[test]
    fn test_render_units() {
        let mut world = core::World::create(core::Map::choke_2p(7));
        world.spawn(core::PlayerID(0));
        world.spawn(core::PlayerID(0));
        world.spawn(core::PlayerID(1));

        let runits = render_units(&world.units);
        assert_eq!(3, runits.x.len());
        assert_eq!(3, runits.y.len());
        assert_eq!(3, runits.owner.len());
        assert_eq!(0, runits.owner[0]);
        assert_eq!(0, runits.owner[1]);
        assert_eq!(1, runits.owner[2]);
    }
}
