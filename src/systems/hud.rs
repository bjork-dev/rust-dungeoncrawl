use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Name)]
pub fn hud(world: &SubWorld) {
    let mut health_query = <&Health>::query().filter(component::<Player>());

    let player_health = health_query.iter(world).nth(0).unwrap();

    let mut draw_batch = DrawBatch::new();

    let (player, map_level) = <(Entity, &Player)>::query()
        .iter(world)
        .find_map(|(entity, _player)| Some((*entity, _player.map_level)))
        .unwrap();

 

    let mut item_query = <(&Item, &Name, &Carried)>::query();
    let mut y = 3;

    item_query
        .iter(world)
        .filter(|(_, _, carried)| carried.0 == player)
        .for_each(|(_, name, _)| {
            draw_batch.print(Point::new(3, y), format!("{} : {}", y - 2, &name.0));
            y += 1;
        });

    if y > 3 {
        draw_batch.print_color(
            Point::new(3, 2),
            "Items carried",
            ColorPair::new(YELLOW, BLACK),
        );
    }

    draw_batch.target(2);
    draw_batch.print_centered(
        1,
        format!(
            "Level {} Explore the dungeon. Cursor keys to move.",
            map_level
        ),
    );

    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );

    draw_batch.print_color_centered(
        0,
        format!(" Health {} / {} ", player_health.current, player_health.max),
        ColorPair::new(WHITE, RED),
    );

    draw_batch.print_color_right(
        Point::new(SCREEN_WIDTH * 2, 1),
        map_level + 1,
        ColorPair::new(YELLOW, BLACK),
    );

    draw_batch.submit(10000).expect("bATCH eRROR")
}
