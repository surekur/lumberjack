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
//use iconhandler::Icons;
use crate::iconhandler::TC;
use crate::iconhandler::Icons;

pub type ListView = Vec<Rc<RefCell<FSnode>>>;

pub trait Manipulate {
    fn close(&mut self, _list_view: &mut ListView, _pos: usize) {}
}


pub trait Listable {
    fn draw(&self, sdl: &mut SdlContainer, font: &Font, pos: (i32, i32), icons: &Icons) -> i32 ;
    fn get_height(&self) -> i32 {20}
}


#[derive(Debug)]
pub enum FSnode {
    DirLike(DirLike),
    Leaf(Leaf),
}
impl FSnode {
    pub fn open<'w>(&mut self, list_view: &mut Vec<Rc<RefCell<FSnode>>>, pos: usize, tc: &'w TC, iconhandler: &mut Icons<'w>) {
        match self {
            Self::DirLike(d) => {if !d.opened {d.open(list_view, pos, tc, iconhandler)}},
            Self::Leaf(f) => {println!("File Open")}, // TODO implement XDG open
        }
    }

    pub fn get_indent(&self) -> i32 {
        match self {
            Self::DirLike(d) => {d.indent},
            Self::Leaf(f) => {f.indent},
        }
    }
}
impl Manipulate for FSnode {
    fn close(&mut self, list_view: &mut Vec<Rc<RefCell<FSnode>>>, pos: usize) {
        match self {
            Self::DirLike(d) => {if d.opened {d.close(list_view, pos);}},
            _ => {}
        }
    }
}
impl Listable for FSnode {
    fn draw(&self, sdl: &mut SdlContainer, font:&Font, pos: (i32, i32), icons: &Icons) -> i32 {
        let pos = match self {
            Self::DirLike(d) => {d.draw(sdl, font, pos, icons)},
            Self::Leaf(f) => {f.draw(sdl, font, pos, icons)},
        };
        pos
    }
    
    fn get_height(&self) -> i32 {
        20
    }
}

#[derive(Debug)]
pub struct DirLike {
    pub name: String,
    pub path: String,
    pub icon: usize,
    //parrent: Option<&'p DirLike<'p,'p>>, // TODO: Use RefCell, Reference counter etc...
    pub meta: Metadata,
    //children: Vec<Rc<RefCell<FSnode>>>,
    pub opened: bool,
    pub indent: i32,
}
impl DirLike {
    pub fn open<'w>(&mut self, list_view: &mut Vec<Rc<RefCell<FSnode>>>, pos: usize, tc: &'w TC, iconhandler: &mut Icons<'w>) {
        self.opened = true; //TODO IMPORTANT investigate: why the opened = true
                            //  not working sometimes if it is on the end of the
                            //  function!
        for file in fs::read_dir(&self.path).unwrap() {
            let file = file.unwrap();
            let meta = file.metadata().unwrap();
            let path = file.path().into_os_string().into_string().unwrap();
            let icon = iconhandler.get_icon(file.path().as_path(), &meta, &tc);

            let node = if file.file_type().unwrap().is_dir() {
                FSnode::DirLike(DirLike{
                    name: file.file_name()
                        .into_string()
                        .unwrap(), // TODO If filename isn't utf-8 encoded, then we panic, is it ok?
                    path: path,
                    icon: icon,
                    meta: meta,
                    //parrent: Some(& self),
                    //children: Vec::new(),
                    opened: false,
                    indent: self.indent+1,
                })
            }
            else {
                
                FSnode::Leaf(Leaf{
                    name: file.file_name().into_string().unwrap(),
                    path: path,
                    meta: meta,
                    icon: icon,
                    indent: self.indent+1,
                })
            };
            let node = Rc::new(RefCell::new(node));
            if pos+1 < list_view.len() {
                list_view.insert(pos+1, node);
                // TODO future Gergo will optimize this
                // so dont need to shift da shit for every single insertion but insert once!
            } else {
                list_view.push(node);
            }
        }
        self.opened = true;
    }
}
impl Manipulate for DirLike {
    fn close(&mut self, list_view: &mut Vec<Rc<RefCell<FSnode>>>, pos: usize) {
        let mut end = pos;
        let mut is_breaked = false;
        if pos+1 < list_view.len() {
            for (i, node) in list_view[pos+1..].iter().enumerate() {
                if node.borrow().get_indent() <= self.indent {
                    end += i+1;
                    is_breaked = true;
                    break
                }
            }
            if !is_breaked {
                end = list_view.len();
                dbg!(!is_breaked, end);
            }
            else {
                //length += pos+1;
            }
            dbg!(pos+1..end);
            list_view.drain(pos+1..end);
        }
        self.opened = false;
    }
}
impl Listable for DirLike {
    fn draw(&self, sdl: &mut SdlContainer, font: &Font, pos: (i32, i32), icons: &Icons) -> i32 {
        sdl.canvas.copy(&icons.loaded[self.icon], None, 
                        Rect::new(self.indent*40+22, pos.1+2, 16, 16)).ok();
        let gpos = ((pos.0 as i16) + (self.indent as i16 *40), pos.1 as i16);
        if !self.opened {
            sdl.canvas.filled_trigon(
                gpos.0+8, gpos.1+2,
                        gpos.0+14, gpos.1+10,
                gpos.0+8, gpos.1+18,
                Color::RGB(255,255,255)
            ).ok();
        }
        else {
            sdl.canvas.filled_trigon(
                gpos.0+2, gpos.1+8,  gpos.0+18, gpos.1+8,
                        gpos.0+10, gpos.1+14,
                Color::RGB(255,255,255)
            ).ok();
        }
        sdl.draw_txt(&self.name, (self.indent*40+pos.0+40, pos.1), &font);
        pos.1 + 20
    }
}


#[derive(Debug)]
pub struct Leaf {
    pub name: String,
    //parrent: Option<&'a DirLike<'a>>,
    pub meta: Metadata,
    pub path: String,
    pub indent: i32,
    pub icon: usize,
}
impl Listable for Leaf {
    fn draw(&self, sdl: &mut SdlContainer, font: &Font, pos: (i32, i32), icons: &Icons) -> i32 {
        sdl.canvas.copy(&icons.loaded[self.icon], None, 
                        Rect::new(self.indent*40+22, pos.1+2, 16, 16)).ok();
        sdl.draw_txt(&self.name, (self.indent*40+40, pos.1), &font);
        pos.1 + 20
    }
}


pub struct Thumbnailable {
    pub name: String,
    //parrent: Option<&'a DirLike<'a>>,
    pub meta: Metadata,
    //thumbnail: 
    pub indent: i32,
}


pub struct SdlContainer {
   pub canvas: sdl2::render::WindowCanvas,
   pub context: sdl2::Sdl,
   pub event_sender: sdl2::event::EventSender,
}
impl SdlContainer {
    pub fn draw_txt(&mut self, txt: &str, pos: (i32, i32), font: &Font ) {
        let surf = font.render(txt)
            .blended(Color::RGB(255,255,255))
            .unwrap();
        let texture_creator = self.canvas.texture_creator();
        let texture = sdl2::render::Texture::from_surface(&surf, &texture_creator)
            .unwrap();
        let size = surf.size();
        self.canvas.copy(&texture, None, Rect::new(pos.0, pos.1, size.0, size.1))
            .expect("yay thats a bug in draw_txt(), yay!");
    }
}

