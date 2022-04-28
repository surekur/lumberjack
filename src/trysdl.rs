extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::ttf::Font;
use std::time::Duration;
use std::path::Path;
use sdl2::rect::Rect;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context: sdl2::ttf::Sdl2TtfContext = sdl2::ttf::init()
        .unwrap();
    let font = ttf_context.load_font(
        Path::new("/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf"),
        15).unwrap();
    //dbg!(ttf_context);
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    //let mut surf = canvas.surface_mut();
    //canvas
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    // Test render
    //let surf;
    let texture_creator = canvas.texture_creator();
    let testtext = font.render("Helló világ!");
    let testtext = testtext.blended(Color::RGB(255,0,0));
    let testtext = testtext.unwrap();
    let testtext = sdl2::render::Texture::from_surface(&testtext, &texture_creator).unwrap();
    canvas.copy(&testtext, Rect::new(0, 0, 100, 100), Rect::new(0, 0, 100, 100)).expect("Yay, no good yay!");

    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        //canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

//fn drawtest(&canvas: sdl2::render::Canvas<sdl2::render::Window>, &font : Font) {
//   let surf = canvas.surface_mut();
   

//}


