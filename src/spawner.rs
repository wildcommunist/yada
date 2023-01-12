use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, position: Point) {
    ecs.push(
        (
            PLayer{},
            position,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@'),
            }
        )
    );
}