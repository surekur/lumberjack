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
//use iconhandler::Icons;


pub trait Manipulate {
    fn close(&mut self, list_view: &mut Vec<FSnode>, pos: usize) {}
}


pub trait Listable {
    fn draw(&self, sdl: &mut SdlContainer, font: &Font, pos: (i32, i32)) -> i32 ;
    fn get_height(&self) -> i32 {20}
}   


#[derive(Debug)]
pub enum FSnode {
    DirLike(DirLike),
    Leaf(Leaf),
}
impl FSnode {
    pub fn open(&mut self, list_view: &mut Vec<FSnode>, pos: usize) {
        match self {
            Self::DirLike(d) => {if !d.opened {d.open(list_view, pos)}},
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
    fn close(&mut self, list_view: &mut Vec<FSnode>, pos: usize) {
        match self {
            Self::DirLike(d) => {if d.opened {d.close(list_view, pos);}},
            _ => {}
        }
    }
}
impl Listable for FSnode {
    fn draw(&self, sdl: &mut SdlContainer, font:&Font, pos: (i32, i32)) -> i32 {
        let pos = match self {
            Self::DirLike(d) => {d.draw(sdl, font, pos)},
            Self::Leaf(f) => {f.draw(sdl, font, pos)},
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
    //parrent: Option<&'p DirLike<'p,'p>>, // TODO: Use RefCell, Reference counter etc...
    pub meta: Metadata,
    //children: Vec<FSnode>,
    pub opened: bool,
    pub indent: i32,
}
impl DirLike {
    pub fn open(&mut self, list_view: &mut Vec<FSnode>, pos: usize) {
        self.opened = true; //TODO IMPORTANT investigate: why the opened = true
                            //  not working sometimes if it is on the end of the
                            //  function!
        for file in fs::read_dir(&self.path).unwrap() {
            let file = file.unwrap();
            let node = if file.file_type().unwrap().is_dir() {
                FSnode::DirLike(DirLike{
                    name: file.file_name().into_string().unwrap(),
                    path: file.path().into_os_string().into_string().unwrap(),
                    meta: file.metadata().unwrap(),
                    //parrent: Some(& self),
                    //children: Vec::new(),
                    opened: false,
                    indent: self.indent+1,
                })
            }
            else {
                
                FSnode::Leaf(Leaf{
                    name: file.file_name().into_string().unwrap(),
                    path: file.path().into_os_string().into_string().unwrap(),
                    meta: file.metadata().unwrap(),
                    indent: self.indent+1,
                })
            };
            if pos+1 < list_view.len() {
                list_view.insert(pos+1, node);
                // TODO optimize so dont need to shift for every single insertion but insert once!
            } else {
                list_view.push(node);
            }
        }
        self.opened = true;
    }
}
impl Manipulate for DirLike {
    fn close(&mut self, list_view: &mut Vec<FSnode>, pos: usize) {
        let mut length = 0;
        let mut is_breaked = false;
        if pos+1 < list_view.len() {
            for (i, node) in list_view[pos+1..].iter().enumerate() {
                if node.get_indent() <= self.indent {
                    length = i;
                    is_breaked = true;
                    break
                }
            }
            if !is_breaked {
                length = list_view.len()+1;
            }
            list_view.drain(pos+1..pos+1+length);
        }
        self.opened = false;
    }
}
impl Listable for DirLike {
    fn draw(&self, sdl: &mut SdlContainer, font: &Font, pos: (i32, i32)) -> i32 {
        let gpos = (pos.0 as i16, pos.1 as i16);
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
        sdl.draw_txt(&self.name, (self.indent*40+pos.0+20, pos.1), &font);
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
}
impl Listable for Leaf {
    fn draw(&self, sdl: &mut SdlContainer, font: &Font, pos: (i32, i32)) -> i32 {
        sdl.draw_txt(&self.name, (self.indent*40+20, pos.1), &font);
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


pub struct SdlContainer<'a> {
   pub canvas: sdl2::render::WindowCanvas,
   pub context: sdl2::Sdl,
   pub event_sender: sdl2::event::EventSender,
   pub texturecreator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
}
impl SdlContainer<'_> {
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

