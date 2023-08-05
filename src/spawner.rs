use crate::prelude::*;

pub fn spawn_amulet_of_yala(ecs : &mut World, pos : Point) {
    ecs.push(
    (Item, AmuletOfYala,
    pos,
    Render{
    color: ColorPair::new(WHITE, BLACK),
    glyph : to_cp437('|')
    },
    Name("Amulet of Yala".to_string())
));
}

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health { current: 20, max: 20 },
        FieldOfView::new(8)
    ));
}

pub fn spawn_monster(ecs: &mut World, pos: Point, rng: &mut RandomNumberGenerator) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };

    ecs.push((
        Enemy,
        ChasingPlayer,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
        FieldOfView::new(6)
    ));
}

fn goblin() -> (i32, String, FontCharType) {
    return (1, "Goblin".to_string(), to_cp437('g'));
}

fn orc() -> (i32, String, FontCharType) {
    return (2, "Orc".to_string(), to_cp437('o'));
}
