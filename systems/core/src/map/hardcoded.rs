use crate::prelude::*;
use itertools::Itertools;
use skyless_core::{prelude::*, World};

system! {
    #[effect(SystemsLoadedEvent)]
    fn load_hardcoded_map(_: EventType, _: GameAttributesType, _: WorldType) -> EffectResultType {
        let mut tiles = HashMap::new();

        const CENTER: u8 = 128;
        const RANGE: u8 = 3;
        const FLOOR: u8 = 7;

        let range = CENTER - RANGE .. CENTER + RANGE + 1;
        let iter = range.clone().cartesian_product(range).map(|(x, y)| (x, y, FLOOR));

        for (x, y, z) in iter {
            let position = Position(x.into(), y.into(), z);
            let mut entities = Vec::new();
            if x == 126 && y == 126 {
                entities.push(entity![Item(Items::StoneSwitch.into()), Action(Actions::Switch)]);
            } else {
                entities.push(entity![Item(Items::Grass.into())]);
                if x == 130 && y == 126 {
                    entities.push(entity![Item(Items::LeverLeft.into()), Action(Actions::Lever)]);
                }
            }
            let tile = Tile {
                attributes: HashMap::new(),
                entities,
            };
            tiles.insert(position, tile);
        }

        Some((vec![Box::new(SetWorldCommand(World(tiles)))], Vec::new()))
    }
}
