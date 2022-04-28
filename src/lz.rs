
use sdl2::ttf::Font;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::os::unix;
use std::path::Path;
use std::fs;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref (A, B) :(i32, i32 ) = {(1, 1)}
    static ref sdl_context: Mutex<sdl2::Sdl> = Mutex::new(sdl2::init().unwrap());
    static ref ttf_context: Mutex<sdl2::ttf::Sdl2TtfContext> = Mutex::new(sdl2::ttf::init()
        .unwrap());
    static ref font: Mutex<Font<'static,'static>>= Mutex::new(ttf_context.load_font(
        Path::new("/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf"),
        15).unwrap());

    static ref video_subsystem: sdl2::VideoSubsystem = sdl_context.video().unwrap();
    static ref window: sdl2::video::Window = video_subsystem.window("LumberJack /home/tesztenv/", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    static ref canvas: sdl2::render::WindowCanvas = window.into_canvas().build().unwrap();
    
}


fn main() {
    draw_txt("Yay! It's werks!", (20,20));
}


fn draw_txt(txt: &str, pos: (i32, i32)) {
    let surf = font.render(txt)
        .blended(Color::RGB(255,255,255))
        .unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = sdl2::render::Texture::from_surface(&surf, &texture_creator)
        .unwrap();
    let size = surf.size();
    canvas.copy(&texture, None, Rect::new(pos.0, pos.1, size.0, size.1))
        .expect("yay thats a bug in draw_txt(), yay!");
}
