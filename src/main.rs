extern crate rand;
extern crate sdl2;

use rand::{thread_rng, Rng};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::log;
use sdl2::rect::Rect;
use std::process;

fn main() {
    log::log("Starting flipper");

    // boundaries
    let height: i32 = 480;
    let width: i32 = 640;

    // random number generator
    let mut rng = thread_rng();

    // SDL context
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // main window
    let window = match video
        .window("Colours", width as u32, height as u32)
        .position_centered()
        .opengl()
        .build() {
        Ok(window) => window,
        Err(err) => panic!("Failed to create window: {}", err),
    };

    // window renderer
    let mut renderer = match window.into_canvas().build() {
        Ok(renderer) => renderer,
        Err(err) => panic!("Failed to create renderer {}", err),
    };

    // moving thing
    let mut thing = Rect::new(10, 10, 10, 10);
    let board = Rect::new(0, 0, width as u32, height as u32);

    // target
    let target_size: u32 = 100;
    let target_x: i32 = rng.gen_range(0, width - target_size as i32);
    let target_y: i32 = rng.gen_range(0, height - target_size as i32);
    let mut target = Rect::new(target_x, target_y, target_size, target_size);

    // colours
    let black = sdl2::pixels::Color::RGB(0, 0, 0);
    let white = sdl2::pixels::Color::RGB(255, 255, 255);
    let gray = sdl2::pixels::Color::RGB(128, 128, 128);
    let dark_gray = sdl2::pixels::Color::RGB(78, 78, 78);

    let mut main_loop = || {
        // events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } |
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    // quit
                    log::log("Exiting flipper");
                    process::exit(1);
                }
                Event::KeyDown { keycode: Some(Keycode::Left), .. } |
                Event::KeyDown { keycode: Some(Keycode::H), .. } => {
                    // move thing left
                    thing.x -= 10;
                    if thing.x < 10 {
                        thing.x = 0;
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } |
                Event::KeyDown { keycode: Some(Keycode::L), .. } => {
                    // move thing right
                    thing.x += 10;
                    if thing.x >= width {
                        thing.x = width - 10;
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } |
                Event::KeyDown { keycode: Some(Keycode::K), .. } => {
                    // move thing up
                    thing.y -= 10;
                    if thing.y < 0 {
                        thing.y = 0;
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } |
                Event::KeyDown { keycode: Some(Keycode::J), .. } => {
                    // move thing down
                    thing.y += 10;
                    if thing.y >= height {
                        thing.y = height - 10;
                    }
                }
                Event::KeyDown { .. } => {
                    // change target if any other key is pressed
                    target.x = rng.gen_range(0, width - target_size as i32);
                    target.y = rng.gen_range(0, height - target_size as i32);
                }
                _ => {}
            }
        }

        // flip colours if thing is inside the target
        let mut inside = false;

        if target.x <= thing.x && target.w + target.x >= thing.x + thing.w &&
            target.y <= thing.y &&
            target.h + target.y >= thing.y + thing.h
        {
            inside = true;
        }

        // draw main window
        let _ = renderer.set_draw_color(black);
        let _ = renderer.clear();

        // draw game board
        if inside == true {
            let _ = renderer.set_draw_color(dark_gray);
        } else {
            let _ = renderer.set_draw_color(gray);
        }
        let _ = renderer.fill_rect(board);

        // draw target
        if inside == true {
            let _ = renderer.set_draw_color(black);
        } else {
            let _ = renderer.set_draw_color(white);
        }

        // draw the thing
        let _ = renderer.fill_rect(target);
        if inside == true {
            let _ = renderer.set_draw_color(white);
        } else {
            let _ = renderer.set_draw_color(black);
        }
        let _ = renderer.fill_rect(thing);

        // present
        let _ = renderer.present();
    };

    loop {
        main_loop();
    }
}
