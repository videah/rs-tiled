extern crate tiled;
extern crate ggez;

use tiled::parse_full;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::EventHandler;

struct State {
    map: tiled::Map,
}

impl EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        use ggez::graphics::*;

        clear(ctx);

        // draws the map
        for layer in self.map.layers.iter() {
            if !layer.visible {
                continue;
            }

            for (j, row) in layer.tiles.iter().enumerate() {
                for (i, &gid) in row.iter().enumerate() {
                    if gid == 0 {
                        continue;
                    }
                    
                    let tileset = self.map.get_tileset_by_gid(gid).unwrap();

                    let gid = gid - tileset.first_gid;
                    let ti = gid % (tileset.images[0].width as u32 / tileset.tile_width);
                    let tj = gid / (tileset.images[0].width as u32 / tileset.tile_width);
                    println!("{} {}", ti, tj);
                    println!("{:?}", Rect::new(ti as f32 * tileset.tile_width as f32 / tileset.images[0].width as f32,
                                               tj as f32 * tileset.tile_height as f32 / tileset.images[0].height as f32,
                                               tileset.tile_width as f32 / tileset.images[0].width as f32,
                                               tileset.tile_height as f32 / tileset.images[0].height as f32));

                    draw_ex(ctx, &tileset.images[0],
                            DrawParam {
                                src: Rect::new(ti as f32 * tileset.tile_width as f32 / tileset.images[0].width as f32,
                                               tj as f32 * tileset.tile_height as f32 / tileset.images[0].height as f32,
                                               tileset.tile_width as f32 / tileset.images[0].width as f32,
                                               tileset.tile_height as f32 / tileset.images[0].height as f32),
                                dest: Point2::new(i as f32 * self.map.tile_width as f32,
                                                  j as f32 * self.map.tile_height as f32),
                                .. Default::default()
                            }
                    ).unwrap();
                }
            }
        }

        present(ctx);
        Ok(())
    }
}

fn main() {
    let mut ctx = ContextBuilder::new("rs-tiled", "Difarem")
        .window_setup(WindowSetup::default()
                      .title("rs-tiled + ggez demo")
                      )
        .window_mode(WindowMode::default())
        .build().unwrap();

    let map = parse_full(&mut ctx, "/tiled_base64_zlib.tmx").unwrap();

    let mut state = State {
        map,
    };
    if let Err(e) = ggez::event::run(&mut ctx, &mut state) {
        println!("error encountered: {}", e);
    } else {
        println!("exited cleanly!");
    }
}
