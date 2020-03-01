#![windows_subsystem = "windows"]

use fae::{Context, Font, GraphicsContext, Image, SpritesheetBuilder};
use sdl2::event::{Event, WindowEvent};
use std::time::{Duration, Instant};

mod error;
mod input;
mod sprites;
mod ui;
mod world;

use error::Logger;
use ui::Ui;
use world::{PlayerAction, World};

static LOGGER: Logger = Logger;
pub static TITLE: &str = "7DRL 2020 by neonmoe";

fn main() -> Result<(), fae::Error> {
    log::set_logger(&LOGGER)
        .map(|_| log::set_max_level(log::LevelFilter::Info))
        .ok(); // set_logger will only fail if a logger has already been set

    let sdl = match sdl2::init() {
        Ok(sdl) => sdl,
        Err(err) => {
            error::user_facing_error(&format!("SDL initialization failed: {}", err), None);
            return Ok(());
        }
    };
    let sdl_video = match sdl.video() {
        Ok(video_subsystem) => video_subsystem,
        Err(err) => {
            error::user_facing_error(
                &format!("SDL video subsystem initialization failed: {}", err),
                None,
            );
            return Ok(());
        }
    };

    let window = sdl_video
        .window(TITLE, 640, 480)
        .opengl()
        .allow_highdpi()
        .resizable()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    fae::gl::load_with(|name| sdl_video.gl_get_proc_address(name) as *const _);

    let mut fae_ctx: Context = Context::new();
    let ttf = include_bytes!("../fonts/libre-baskerville/libre-baskerville.ttf").to_vec();
    let font = Font::with_ttf(&mut fae_ctx, ttf).unwrap();
    let tileset_image = Image::with_png(include_bytes!("tileset.png"))?;
    let tileset = SpritesheetBuilder::default()
        .alpha_blending(true)
        .minification_smoothing(true)
        .magnification_smoothing(true)
        .image(tileset_image)
        .build(&mut fae_ctx);
    let ui_tileset = SpritesheetBuilder::default()
        .alpha_blending(true)
        .build(&mut fae_ctx);

    let mut world = World::new();
    let mut ui = Ui::new();

    let mut event_pump = sdl.event_pump().unwrap();
    let mut last_frame_time = None;

    'game_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'game_loop;
                }
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::Resized(_, _) => unsafe {
                        let (width, height) = window.drawable_size();
                        fae::gl::Viewport(0, 0, width as i32, height as i32);
                    },
                    _ => {}
                },
                Event::KeyDown { keycode, .. } => {
                    if let Some(keycode) = keycode {
                        if input::is_key_move_up(keycode) {
                            world.update(PlayerAction::MoveUp);
                        } else if input::is_key_move_down(keycode) {
                            world.update(PlayerAction::MoveDown);
                        } else if input::is_key_move_right(keycode) {
                            world.update(PlayerAction::MoveRight);
                        } else if input::is_key_move_left(keycode) {
                            world.update(PlayerAction::MoveLeft);
                        } else if input::is_key_wait(keycode) {
                            world.update(PlayerAction::Wait);
                        }
                    }
                }
                _ => {}
            }
        }

        let (width, height) = (window.size().0 as f32, window.size().1 as f32);
        let physical_width = window.drawable_size().0 as f32;
        let dpi_factor = physical_width / width;

        let current_time = Instant::now();
        if let Some(last_frame_time) = last_frame_time {
            let duration: Duration = current_time - last_frame_time;
            let seconds = duration.as_secs_f32();
            world.animate(seconds);
        }
        last_frame_time = Some(current_time);

        fae::profiler::refresh();
        let mut ctx: GraphicsContext = fae_ctx.start_frame(width, height, dpi_factor);

        world.render(&mut ctx, &font, &tileset);
        ui.render(&mut ctx, &font, &ui_tileset);

        ctx.finish_frame();
        fae_ctx.render(width, height, (0.1, 0.1, 0.1, 0.5));
        window.gl_swap_window();
        fae_ctx.synchronize();
    }

    drop(gl_context);
    Ok(())
}