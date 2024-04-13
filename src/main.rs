extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use core::time::Duration;

use volfied_poc::draw::*;

static WINDOW_WIDTH: usize = 800;
static WINDOW_HEIGHT: usize = 600;



fn initialize_windows(sdl_context: &sdl2::Sdl) -> sdl2::video::Window {

    let video_subsystem = sdl_context.video().unwrap();
    
    let window = video_subsystem.
    window("title", 800, 600)
    .position_centered()
    .borderless()
    .build()
    .unwrap();

    window

}



pub fn process_event(sdl_context: &sdl2::Sdl) -> bool {
    let mut event_pump = sdl_context.event_pump().unwrap();
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return false;
            },
            _ => {}
        }
    }
    true
}

pub fn initialize_texture(texture_creator: &TextureCreator<WindowContext>) -> Texture {
    texture_creator
    .create_texture_streaming(
        PixelFormatEnum::ARGB8888,
        WINDOW_WIDTH as u32,
        WINDOW_HEIGHT as u32).unwrap()
}


pub fn render(canvas: &mut Canvas<sdl2::video::Window>, color_buffer_texture: &mut Texture) {

    canvas.clear();
    canvas.copy(color_buffer_texture, None, None).unwrap();
    canvas.present();
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

}

pub fn main() {

    let sdl_context: sdl2::Sdl = sdl2::init().unwrap();

    let window = initialize_windows(&sdl_context);

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();


    let mut color_buffer = ColorBuffer {
        buffer: vec![0; WINDOW_WIDTH * WINDOW_HEIGHT * 4],
        height: WINDOW_HEIGHT,
        width: WINDOW_WIDTH
      };
    
    
    
    let texture_creator = canvas.texture_creator();


    let mut color_buffer_texture = initialize_texture(&texture_creator);

    color_buffer = clear_color_buffer(color_buffer, Color { r: 255, g: 165, b: 0, a: 255 });
    color_buffer = draw_grid(color_buffer, Color { r: 0, g: 0, b: 0, a: 255 });

    //color_buffer = draw_rectangle(color_buffer, 100, 100, 500, 240, Color { r: 255, g: 0, b: 0, a: 255 });

    let mut x = 10;
    let mut y = 10;

    'running: loop {
        if !process_event(&sdl_context) {
            break 'running;
        }
        //i = (i + 1) % 69;
        //color_buffer = clear_color_buffer(color_buffer, Color { r: 255 - i, g: 255, b: 69 - i, a: 0 });
        color_buffer = draw_pixel(color_buffer, x, y, Color { r: 0, g: 0, b: 0, a: 255 });
        color_buffer_texture.update(
            None,
                &color_buffer.buffer,
                WINDOW_WIDTH * 4
            ).unwrap();
        render(&mut canvas, &mut color_buffer_texture);
        if x <= WINDOW_WIDTH - 10 {
            x += 1;
        }
        else {
            x = 10;
            y += 10;
        }

      
    }
}


