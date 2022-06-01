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
use sdl2::gfx::primitives::DrawRenderer;
use std::env;
use iconhandler::Icons;
use crate::fsnodetypes::{SdlContainer, FSnode, DirLike, Leaf, Manipulate, Listable};
use crate::iconhandler::TC;

mod iconhandler;
mod fsnodetypes;
mod modality;

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
    let mut list_view: Vec<FSnode> = Vec::new();
    root_node.open(&mut list_view, 0, &texturecreator, &mut icons);
    let mut cursorpos = 0;
    let mut viewpos = 0;
    let selection: Vec<isize>;

    'main_loop : loop {
        let event = event_pump.wait_event();
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                break 'main_loop
            },
            Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                if cursorpos > 0 {
                    cursorpos -= 1;
                }
            }
            Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                if cursorpos < list_view.len()-1 {
                    cursorpos += 1;
                }
            }
            Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                let node = unsafe {
                    let node = &mut list_view[cursorpos] as *mut FSnode;
                    &mut *node
                };
                node.open(&mut list_view, cursorpos, &texturecreator, &mut icons);
            }
            Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                unsafe {
                    let node = &mut list_view[cursorpos] as *mut FSnode;
                    let node = &mut *node;
                    node.close(&mut list_view, cursorpos);
                }
            }
            Event::KeyDown { keycode: Some(Keycode::Return), ..} => {
                let (path, meta) = match &list_view[cursorpos] {
                    FSnode::DirLike(d) => {dbg!(d);
                        (&d.path, &d.meta)}
                    FSnode::Leaf(f) => {(&f.path, &f.meta)}
                };
                dbg!("Lefut.");
                let meta = meta;
                let icon = icons.get_icon(Path::new(&path), meta, &texturecreator);
            }

            _ => {}//dbg!("Other event!", event);}
        }
        sdl.canvas.clear();
        let mut pos = 0;
        let mut iseven = false;
        let mut last_visible = viewpos;
        let mut breaked = false;
        for (i, entry) in list_view[viewpos..].iter().enumerate() {
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
                .expect("Bug in drawing the background!");
            pos = entry.draw(&mut sdl, &font, (0, pos), &icons);
        }
        sdl.canvas.present();
        if breaked && (last_visible < cursorpos+1) {
            viewpos += 1;
            sdl.event_sender.push_custom_event(BumpEvent {})
                .expect("Nem sikerült!");
        }
        if viewpos > cursorpos {
            viewpos -= 1;
            sdl.event_sender.push_custom_event(BumpEvent {})
                .expect("Nem sikerült!");
        }
    }
}

