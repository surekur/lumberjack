use sdl2::ttf::Font;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
//use sdl2::render::WindowCanvas;


use std::os::unix;
use fs::Metadata;
use std::path::{Path, PathBuf};
use std::fs;

use std::cell::RefCell;
use std::rc::Rc;
use sdl2::gfx::primitives::DrawRenderer;
use std::env;
use iconhandler::Icons;
use crate::fsnodetypes::{FSnode, DirLike, Leaf, Manipulate, ListView};
use crate::iconhandler::TexCre;
use crate::modality::{Mode};
use crate::config::*;
use crate::render::*;
use crate::modality::Command;

mod iconhandler;
mod fsnodetypes;
mod modality;
mod config;
mod commands;
mod render;

struct BumpEvent {}

fn update_dirs(list_view: &ListView, glob: GlobalState) {
    
}

type PosList = Vec<usize>;

struct GlobalState {
    pub viewpos: usize,
    pub cursorpos: usize,
    pub winsize: (u32, u32),
    pub selection: PosList,
    pub openeddirs: Vec<PathBuf>,
    pub mousepos: usize,
}

fn mouse_pos_as_list_index(mousepos: (i32, i32), list_view: &ListView,
                           viewpos: usize, winsize: (u32, u32)) -> Option<usize> {
    let mut pos = 0;
    for (i, entry) in list_view[viewpos..].iter().enumerate() {
        let height = {entry.borrow_mut().get_height()};
        let nextpos = pos + height;
        if (pos + height + 20) > winsize.1 as i32 {
            return None
        }
        if pos <= mousepos.1 && mousepos.1 < nextpos {
            return Some(viewpos + i)
        }
        pos = nextpos;
    }
    None
}


fn main() {
    // Config
    let root_path = "./";
    let bgempty = Color::RGB(15, 15, 15);

    // SDL2 Boilerplate
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context: sdl2::ttf::Sdl2TtfContext = sdl2::ttf::init()
        .unwrap();
    let font = ttf_context.load_font(
        Path::new("/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf"), 15)
        .unwrap();
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

    // Globalstate
    let mut root_node = FSnode::DirLike(DirLike {name: String::from(""),
                                                path: PathBuf::from(root_path),
                                                indent: -1,
                                                meta: fs::metadata(root_path).unwrap(),
                                                opened: false,
                                                icon: 0,
    });
    let mut list_view: ListView = Vec::new();
    root_node.open(&mut list_view, 0, &texturecreator, &mut icons);
    let mut cursorpos = 0;
    let mut viewpos = 0;
    let mut mode = Mode::normal();
    let mut openeddirs: Vec<PathBuf> = Vec::new();
    let mut glob = GlobalState {
        cursorpos: 0,
        viewpos: 0,
        selection: Vec::new(),
        winsize: sdl.canvas.window().drawable_size(),
        openeddirs: Vec::new(),
        mousepos: usize::MAX,
    };

    loop { //MAIN LOOP //////////////

        let mut mousepos = usize::MAX; // Hope you dont have too mutch file...
        // Input
        let event = event_pump.wait_event();
        let winsize = sdl.canvas.window().drawable_size();
        match event {
            Event::MouseMotion{x, y, ..} => {
                if let Some(mp) = mouse_pos_as_list_index((x, y), &list_view, viewpos,
                winsize) {
                    mousepos = mp;
                }
            },
            Event::MouseButtonDown{x, y, ..} => {
                if let Some(mp) = mouse_pos_as_list_index((x, y), &list_view, viewpos,
                winsize) {
                    let node = list_view[mp].clone();
                    let mut node = node.borrow_mut();
                    node.on_click(&mut cursorpos , &mut list_view, event, mp, 
                                  &texturecreator, &mut icons);
                }
            },
            Event::Quit {..} => {break;},
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                mode = Mode::normal();
            },
            _ => {
                (mode.handle_input)(&mut mode, event, &mut cursorpos, &mut list_view,
                                            &mut icons, &texturecreator, &mut openeddirs);
            }
        }
        

        // Render
        sdl.canvas.clear();
        let mut pos = 0;
        let mut iseven = false;
        let mut lastvisible = viewpos;
        let mut breaked = false;
        for (i, entry) in list_view[viewpos..].iter().enumerate() {
            let entry = entry.borrow_mut();
            let list_index = i + viewpos;
            let height = entry.get_height();
            lastvisible += 1;
            if pos + height > (sdl.canvas.window().drawable_size().1 as i32 - 20) {
                lastvisible -= 1;
                breaked = true;
                break;
            }
            if list_index == cursorpos {
                sdl.canvas.set_draw_color(CURSOR_COLOR);
            }
            else if list_index == mousepos {
                sdl.canvas.set_draw_color(MOUSE_HOOVER);
            }
            else if iseven {
                sdl.canvas.set_draw_color(BG0);
            }
            else {
                sdl.canvas.set_draw_color(BG1);
            }
            iseven = !iseven;
            sdl.canvas.fill_rect(Some(Rect::new(
                        LINECOUNTERWIDTH as i32, pos,
                        sdl.canvas.window().drawable_size().0,
                        entry.get_height() as u32))).ok();
            let distancefromcursor = if list_index == cursorpos {
                0
            }
            else if list_index > cursorpos {
                list_index - cursorpos
            }
            else {
                cursorpos - list_index
            };
            draw_linecounter(height as u32, distancefromcursor, pos, &mut sdl, &font, iseven);
            pos = entry.draw(&mut sdl, &font, (LINECOUNTERWIDTH as i32, pos), &icons);
        }
        if !breaked {
                let winsize = sdl.canvas.window().drawable_size();
                let statusbarpos = winsize.1 - (((winsize.1 - 20 - pos as u32 ) % 20)+20) ;
                sdl.canvas.set_draw_color(bgempty); // TODO: rethink, optimize calculations.
                sdl.canvas.fill_rect(Rect::new(0, pos,
                                               winsize.0, statusbarpos - pos as u32)).ok();
                pos = statusbarpos as i32;
        }
        draw_statusbar(pos, cursorpos, &mut list_view, &mut mode, &mut sdl, &font);
        draw_minimap(&list_view, viewpos, lastvisible, &mut sdl, pos as u32, 40);
        sdl.canvas.present();
        if breaked && (lastvisible < cursorpos+1) {
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

