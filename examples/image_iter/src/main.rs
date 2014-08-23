
#![feature(globs)]

extern crate graphics;
extern crate piston;
extern crate sdl2_game_window;
extern crate gfx_graphics;

use gfx_graphics::{
    Gfx2d,
    Texture,
};
use sdl2_game_window::GameWindowSDL2;
use graphics::*;
use piston::{
    AssetStore,
    GameIterator,
    GameIteratorSettings,
    GameWindowSettings,
    Render,
};

fn main() {
    let mut window = GameWindowSDL2::new(
        piston::shader_version::opengl::OpenGL_3_2,
        GameWindowSettings {
            title: "Image".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
        }
    );

    let asset_store = AssetStore::from_folder("../bin/assets");

    let image = asset_store.path("rust-logo.png").unwrap();
    let image = Texture::from_path(&image).unwrap();
    let game_iter_settings = GameIteratorSettings {
            updates_per_second: 120,
            max_frames_per_second: 60,
        };
    let (mut device, _) = window.gfx();
    let ref mut g = Gfx2d::new(&mut device);
    for e in GameIterator::new(&mut window, &game_iter_settings) {
        match e {
            Render(args) => {
                // gl.viewport(0, 0, args.width as i32, args.height as i32);

                let c = Context::abs(args.width as f64, args.height as f64);
                c.rgb(1.0, 1.0, 1.0).draw(g);
                    c.image(&image).draw(g);
            },
            _ => {},
        }
    }
}
