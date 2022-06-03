use sdl2::ttf::Font;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
//use sdl2::render::WindowCanvas;

use std::os::unix;
use fs::Metadata;
use std::path::Path;
use std::fs;

use std::cell::RefCell;
use std::rc::Rc;
use sdl2::gfx::primitives::DrawRenderer;
use std::env;
use iconhandler::Icons;
use crate::fsnodetypes::{SdlContainer, FSnode, DirLike, Leaf, Manipulate, Listable};
use crate::iconhandler::TC;
use crate::modality::{Mode, NormalMode};

mod iconhandler;
mod fsnodetypes;
mod modality;


pub type TextureCreator = sdl2::render::TextureCreator<sdl2::video::WindowContext>;

struct BumpEvent {
}

fn draw_statusbar(pos: usize, cursorpos: usize) {
    

}


fn main() {
    let root_path = "./";
    let bg1 = Color::RGB(20,20,20);
    let bg2 = Color::RGB(25,25,25);
    let cursorcolor = Color::RGB(0, 0, 140);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context: sdl2::ttf::Sdl2TtfContext = sdl2::ttf::init()
        .unwrap();
    let font = ttf_context.load_font(
        Path::new("/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf"), 15)
        .unwrap();
    //dbg!(ttf_context);
    let window = video_subsystem.window(format!("LumberJack {}", root_path).as_str(), 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(20, 20, 20));
    let texturecreator = canvas.texture_creator();
    let event_sender = sdl_context.event().unwrap().event_sender();
    let mut sdl = SdlContainer {
        canvas: canvas,
        context: sdl_context,
        event_sender: event_sender,
    };
    let mut event_pump = sdl.context.event_pump().unwrap();
    sdl.context.event()
        .unwrap()
        .register_custom_event::<BumpEvent>().unwrap();
    let mut icons = Icons::new();

    let mut root_node = FSnode::DirLike(DirLike {name: String::from(""),
                                                path: String::from(root_path),
                                                indent: -1,
                                                meta: fs::metadata(root_path).unwrap(),
                                                opened: false,
                                                icon: 0,
    });
    let mut list_view: Vec<Rc<RefCell<FSnode>>> = Vec::new();
    root_node.open(&mut list_view, 0, &texturecreator, &mut icons);
    let mut cursorpos = 0;
    let mut viewpos = 0;
    let mut mode = Mode::Normal(NormalMode::new());
    let selection: Vec<isize>;

    loop {
        let event = event_pump.wait_event();
        if !match &mut mode {
            Mode::Normal(m) => {m.handle_input(event, &mut cursorpos, &mut list_view,
                                               &mut icons, &texturecreator)},
            _ => {true},

        }
        {break};
        
        sdl.canvas.clear();
        let mut pos = 0;
        let mut iseven = false;
        let mut last_visible = viewpos;
        let mut breaked = false;
        for (i, entry) in list_view[viewpos..].iter().enumerate() {
            let entry = entry.borrow_mut();
            if pos + entry.get_height() > (sdl.canvas.window().drawable_size().1 as i32) {
                last_visible += i;
                breaked = true;
                break;
            }
            if i + viewpos == cursorpos {
                sdl.canvas.set_draw_color(cursorcolor);
            } else if iseven {
                sdl.canvas.set_draw_color(bg2);
            } else {
                sdl.canvas.set_draw_color(bg1);
            }
            iseven = !iseven;
            sdl.canvas.fill_rect(Some(Rect::new(
                        0, pos,
                        sdl.canvas.window().drawable_size().0,entry.get_height() as u32)))
                .expect("Can't draw the background!");
            pos = entry.draw(&mut sdl, &font, (0, pos), &icons);
        }
        sdl.canvas.present();
        if breaked && (last_visible < cursorpos+1) {
            viewpos += 1;
            sdl.event_sender.push_custom_event(BumpEvent {})
                .ok();
        }
        if viewpos > cursorpos {
            viewpos -= 1;
            sdl.event_sender.push_custom_event(BumpEvent {})
                .ok();
        }
    }
}

