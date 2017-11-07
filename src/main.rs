extern crate rand;
extern crate sdl2;

mod audio_waves;

use rand::{random, thread_rng, Rng};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::log;
use sdl2::rect::{Point, Rect};
use std::process;
use std::time::Duration;
use audio_waves::waves;

fn next_move(
    current: Point,
    height: i32,
    width: i32,
    movement: i32,
    size: i32,
) -> Point {
    let mut x: i32;
    let mut y: i32;
    if random() {
        x = current.x + movement;
        if x >= width {
            x = width - size;
        }
    } else {
        x = current.x - movement;
        if x < 0 {
            x = 0
        }
    }
    if random() {
        y = current.y + movement;
        if y >= height {
            y = height - size;
        }
    } else {
        y = current.y - movement;
        if y < 0 {
            y = 0
        }
    }
    return Point::new(x, y);
}

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
    let audio = sdl_context.audio().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // main window
    let window = match video
        .window("Flipper", width as u32, height as u32)
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

    // audio
    let spec_vertical = waves::get_audio_spec(44100);
    let spec_horizontal = waves::get_audio_spec(43100);
    let spec_white_noise = waves::get_audio_spec(42100);

    let playback_left = audio
        .open_playback(None, &spec_horizontal, |spec| {
            waves::SquareWave {
                phase_inc: 180.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25,
            }
        })
        .unwrap();

    let playback_right = audio
        .open_playback(None, &spec_horizontal, |spec| {
            waves::SquareWave {
                phase_inc: 220.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25,
            }
        })
        .unwrap();

    let playback_up = audio
        .open_playback(None, &spec_vertical, |spec| {
            waves::SquareWave {
                phase_inc: 260.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25,
            }
        })
        .unwrap();

    let playback_down = audio
        .open_playback(None, &spec_vertical, |spec| {
            waves::SquareWave {
                phase_inc: 160.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25,
            }
        })
        .unwrap();

    // play white noice
    let playback_white_noise = audio
        .open_playback(
            None,
            &spec_white_noise,
            |_| waves::WhiteNoise { volume: 2.0 },
        )
        .unwrap();

    // moving thing
    let mut thing = Rect::new(10, 10, 10, 10);

    // opponent
    let opponent_x: i32 = rng.gen_range(15, width - 10);
    let opponent_y: i32 = rng.gen_range(15, height - 10);
    let mut opponent = Rect::new(opponent_x, opponent_y, 10, 10);

    // playing area
    let board = Rect::new(0, 0, width as u32, height as u32);

    // target
    let target_size: u32 = 100;
    let target_x: i32 = rng.gen_range(0, width - target_size as i32);
    let target_y: i32 = rng.gen_range(0, height - target_size as i32);
    let mut target = Rect::new(target_x, target_y, target_size, target_size);

    // colours
    let black = sdl2::pixels::Color::RGB(0, 0, 0);
    let white = sdl2::pixels::Color::RGB(255, 255, 255);
    let light_gray = sdl2::pixels::Color::RGB(200, 200, 200);
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
                    // play sound
                    playback_left.resume();
                    std::thread::sleep(Duration::from_millis(200));
                    playback_left.pause();
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } |
                Event::KeyDown { keycode: Some(Keycode::L), .. } => {
                    // move thing right
                    thing.x += 10;
                    if thing.x >= width {
                        thing.x = width - 10;
                    }
                    // play sound
                    playback_right.resume();
                    std::thread::sleep(Duration::from_millis(200));
                    playback_right.pause();
                }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } |
                Event::KeyDown { keycode: Some(Keycode::K), .. } => {
                    // move thing up
                    thing.y -= 10;
                    if thing.y < 0 {
                        thing.y = 0;
                    }
                    // play sound
                    playback_up.resume();
                    std::thread::sleep(Duration::from_millis(200));
                    playback_up.pause();
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } |
                Event::KeyDown { keycode: Some(Keycode::J), .. } => {
                    // move thing down
                    thing.y += 10;
                    if thing.y >= height {
                        thing.y = height - 10;
                    }
                    // play sound
                    playback_down.resume();
                    std::thread::sleep(Duration::from_millis(200));
                    playback_down.pause();
                }
                Event::KeyDown { .. } => {
                    // change target if any other key is pressed
                    target.x = rng.gen_range(0, width - target_size as i32);
                    target.y = rng.gen_range(0, height - target_size as i32);
                }
                _ => {
                    // move opponent
                    let current_point = opponent.top_left();
                    opponent.reposition(
                        next_move(current_point, height, width, 10, 10),
                    );
                }
            }
        }

        // check if game over
        if target.x <= opponent.x &&
            target.w + target.x >= opponent.x + opponent.w &&
            target.y <= opponent.y &&
            target.h + target.y >= opponent.y + opponent.h
        {
            // game over
            log::log("Game over");
            process::exit(1);
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
            // play white noise
            playback_white_noise.resume();
        } else {
            let _ = renderer.set_draw_color(gray);
            // pasue white noise
            playback_white_noise.pause();
        }
        let _ = renderer.fill_rect(board);

        // draw target
        if inside == true {
            let _ = renderer.set_draw_color(black);
        } else {
            let _ = renderer.set_draw_color(white);
        }
        let _ = renderer.fill_rect(target);

        // draw the thing
        if inside == true {
            let _ = renderer.set_draw_color(white);
        } else {
            let _ = renderer.set_draw_color(black);
        }
        let _ = renderer.fill_rect(thing);

        // draw the opponent
        let _ = renderer.set_draw_color(light_gray);
        let _ = renderer.fill_rect(opponent);

        // present
        let _ = renderer.present();
    };

    loop {
        main_loop();
    }
}
