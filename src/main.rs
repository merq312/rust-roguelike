use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;

pub use components::*;
pub use map::*;
pub use player::*;
pub use rect::Rect;

mod components;
mod map;
mod player;
mod rect;

pub struct State {
    ecs: World,
}

// impl State {
//     fn run_systems(&mut self) {
//         let mut lw = LeftWalker {};
//         lw.run_now(&self.ecs);
//         self.ecs.maintain();
//     }
// }

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        // ctx.print(1, 1, "Hello Rust World");

        player_input(self, ctx);
        // self.run_systems();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    // gs.ecs.register::<LeftMover>();

    let (rooms, map) = new_map_rooms_and_corridors();
    gs.ecs.insert(map);
    let (player_x, player_y) = rooms[0].center();

    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    // for i in 0..10 {
    //     gs.ecs
    //         .create_entity()
    //         .with(Position { x: i * 7, y: 20 })
    //         .with(Renderable {
    //             glyph: rltk::to_cp437('☺'),
    //             fg: RGB::named(rltk::RED),
    //             bg: RGB::named(rltk::BLACK),
    //         })
    //         // .with(LeftMover {})
    //         .build();
    // }

    rltk::main_loop(context, gs)
}
