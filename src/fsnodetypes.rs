use sdl2::ttf::Font;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
//use sdl2::render::WindowCanvas;
use crate::GlobalState;

use std::os::unix;
use fs::Metadata;
use std::path::{Path, PathBuf};
use std::fs;

use std::cell::RefCell;
use std::rc::Rc;
use sdl2::gfx::primitives::DrawRenderer;
use std::env;
//use iconhandler::Icons;
use crate::iconhandler::TexCre;
use crate::iconhandler::Icons;
use crate::config::*;
use std::cmp::Ordering;

pub type ListView = Vec<Rc<RefCell<FSnode>>>;
pub type PackedFSnode = Rc<RefCell<FSnode>>;

pub trait Manipulate {
    fn update<'w>(&mut self, _pos: usize, _list_view: &mut ListView,
              _tc: &'w TexCre, _iconhandler: &mut Icons<'w>) {}
    fn close(&mut self, _list_view: &mut ListView, _pos: usize) {}
    fn open<'w>(&mut self, _list_view: &mut ListView, _pos: usize, _tc: &'w TexCre, _iconhandler: &mut Icons<'w>) {}
    fn get_indent(&self) -> i32 {0}
    //-----------------------------------------------------------------
    fn get_name(&self) -> &str {""}

    fn get_path(&self, list_view: &ListView, pos: usize) -> Option<PathBuf> {
        Some(PathBuf::new())
    }

    fn get_parrent(&self, list_view: &ListView, pos: usize) -> Option<usize> {
        if self.get_indent() == 0 {
            return None;
        }
        let mut i = pos;
        loop {
            i -= 1;
            if list_view[i].borrow().get_indent() < self.get_indent() {
                return Some(i);
            }
        }
    }

    fn on_click<'w>(&mut self, glob:  &mut GlobalState, list_view: &mut ListView, event: Event,
                undermouse: usize, tc: &'w TextureCreator, iconhandler: &mut Icons<'w> ) {
        if let Event::MouseButtonDown{x, y, clicks, ..} = event {
            if clicks == 1 {
                glob.cursorpos = undermouse;
            }
            else if clicks == 2 {
                self.open(list_view, undermouse, tc, iconhandler);
            }
        }
    }
}




#[derive(Debug)]
pub enum FSnode {
    DirLike(DirLike),
    Leaf(Leaf),
}
impl FSnode {

}
impl Manipulate for FSnode {
    fn update<'w>(&mut self, pos: usize, list_view: &mut ListView,
              tc: &'w TexCre, iconhandler: &mut Icons<'w>) {
        match self {
            Self::DirLike(d) => {d.update(pos, list_view, tc, iconhandler)},
            Self::Leaf(f) => {f.update(pos, list_view, tc, iconhandler)},
        }
    }
    fn get_name(&self) -> &str {
        match self {
            Self::DirLike(d) => {&d.name},
            Self::Leaf(f) => {&f.name},
        }
    }

    fn get_indent(&self) -> i32 {
        match self {
            Self::DirLike(d) => {d.indent},
            Self::Leaf(f) => {f.indent},
        }
    }

    fn close(&mut self, list_view: &mut ListView, pos: usize) {
        match self {
            Self::DirLike(d) => {if d.opened {d.close(list_view, pos);}},
            _ => {}
        }
    }

    fn open<'w>(&mut self, list_view: &mut ListView, pos: usize, tc: &'w TexCre, iconhandler: &mut Icons<'w>) {
        match self {
            Self::DirLike(d) => {if !d.opened {d.open(list_view, pos, tc, iconhandler)}},
            Self::Leaf(f) => {println!("File Open")}, // TODO implement XDG open
        }
    }
}

#[derive(Debug)]
pub struct DirLike {
    pub name: String,
    pub path: PathBuf,
    pub icon: usize,
    //parrent: Option<&'p DirLike<'p,'p>>, // TODO: Use RefCell, Reference counter etc...
    pub meta: Metadata,
    //children: Vec<Rc<RefCell<FSnode>>>,
    pub opened: bool,
    pub indent: i32,
}
impl DirLike {
}
impl Manipulate for DirLike {
    fn get_name(&self) -> &str {&self.name}
    fn get_indent(&self) -> i32 {self.indent}

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
            }
            else {
                //length += pos+1;
            }
            list_view.drain(pos+1..end);
        }
        self.opened = false;
    }

    fn open<'w>(&mut self, list_view: &mut ListView, pos: usize, tc: &'w TexCre, iconhandler: &mut Icons<'w>) {
        // TODO Handle fails properly!
        let dirread = fs::read_dir(&self.path).unwrap();

        let mut children = Vec::with_capacity(200); // TODO get somehow de size of the dir
        for file in dirread {
            let file = file.unwrap();
            let meta = file.metadata().unwrap();
            let path = file.path().into_os_string().into_string().unwrap();
            let icon = iconhandler.get_icon(file.path().as_path(), &meta, &tc);

            let node = if file.file_type().unwrap().is_dir() {
                FSnode::DirLike(DirLike{
                    name: file.file_name()
                        .into_string()
                        .unwrap(), // TODO If filename isn't utf-8 encoded, then we panic, is it ok?
                    path: file.path(),
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
          //  if pos+1 < list_view.len() {
          //      list_view.insert(pos+1, node);
          //      // TODO future Gergo will optimize this
          //      // so dont need to shift da shit for every single insertion but insert once!
          //  } else {
          //      list_view.push(node);
          //  }
            children.push(node);
        }
        children.sort_by(orderer);
        if !children.is_empty() {
            if list_view.is_empty() {
                std::mem::swap(list_view, &mut children);
            }
            // adding a dummynode...
            else {
                if pos+1 < list_view.len() {
                    list_view.insert(pos+1, children[0].clone());
                }
                else {
                    list_view.push(children[0].clone());
                }
                dbg!(pos+1..=pos+1);
                    list_view.splice(pos+1..=pos+1, children);
            }
        }
        self.opened = true;
    }

    fn update<'w>(&mut self, pos: usize, list_view: &mut ListView,
              tc: &'w TexCre, iconhandler: &mut Icons<'w>) {
        // TODO It is extremly inefective this way.
        self.close(list_view, pos);
        self.open(list_view, pos, tc, iconhandler);
        // TODO opened dirs inside shoud be still opened.

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
impl Manipulate for Leaf {
    fn get_indent(&self) -> i32 {self.indent}
    fn get_name(&self) -> &str {&self.name}
}


pub struct Thumbnailable {
    pub name: String,
    //parrent: Option<&'a DirLike<'a>>,
    pub meta: Metadata,
    //thumbnail: 
    pub indent: i32,
}

// TODO Order by different fields?
fn orderer(first: &PackedFSnode, second: &PackedFSnode) -> Ordering {
    let first = &*first.borrow();
    let second = &*second.borrow();
    let (firstname, firstisdir) = match first {
        FSnode::DirLike(f) => {(&f.name, true)},
        FSnode::Leaf(f) => {(&f.name, false)},
    };
    match second {
        FSnode::DirLike(f) => {
            if !firstisdir {
                return Ordering::Greater;
            }
            return firstname.cmp(&f.name);
        },
        FSnode::Leaf(f) => {
            if firstisdir {
                return Ordering::Less;
            }
            return firstname.cmp(&f.name);
        },
    }
}


