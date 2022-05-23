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
use freedesktop_icons::lookup;

trait Manipulate {
    fn close(&mut self, list_view: &mut Vec<Box<FSnode>>, pos: usize) {}
}


trait Listable {
    fn draw(&self, sdl: &mut SdlContainer, font: &Font, pos: (i32, i32)) -> i32 ;
    fn get_height(&self) -> i32 {20}
}


enum FSnode {
    DirLike(DirLike),
    Leaf(Leaf),
}
impl FSnode {
    fn open(&mut self, list_view: &mut Vec<Box<FSnode>>, pos: usize) {
        match self {
            Self::DirLike(d) => {if !d.opened {d.open(list_view, pos)}},
            Self::Leaf(f) => {println!("File Open")}, // TODO implement XDG open
        }
    }

    fn get_indent(&self) -> i32 {
        match self {
            Self::DirLike(d) => {d.indent},
            Self::Leaf(f) => {f.indent},
        }
    }
}
impl Manipulate for FSnode {
    fn close(&mut self, list_view: &mut Vec<Box<FSnode>>, pos: usize) {
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

struct DirLike {
    name: String,
    path: String,
    //parrent: Option<&'p DirLike<'p,'p>>, // TODO: Use RefCell, Reference counter etc...
    meta: Metadata,
    //children: Vec<FSnode>,
    opened: bool,
    indent: i32,
}
impl DirLike {
    fn open(&mut self, list_view: &mut Vec<Box<FSnode>>, pos: usize) {
        for file in fs::read_dir(&self.path).unwrap() {
            let file = file.unwrap();
            let node = if file.file_type().unwrap().is_dir() {
                Box::new(FSnode::DirLike(DirLike{
                    name: file.file_name().into_string().unwrap(),
                    path: file.path().into_os_string().into_string().unwrap(),
                    meta: file.metadata().unwrap(),
                    //parrent: Some(& self),
                    //children: Vec::new(),
                    opened: false,
                    indent: self.indent+1,
                }))
            }
            else {
                
                Box::new(FSnode::Leaf(Leaf{
                    name: file.file_name().into_string().unwrap(),
                    path: file.path().into_os_string().into_string().unwrap(),
                    meta: file.metadata().unwrap(),
                    indent: self.indent+1,
                }))
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
    fn close(&mut self, list_view: &mut Vec<Box<FSnode>>, pos: usize) {
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
        } else {
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


struct Leaf {
    name: String,
    //parrent: Option<&'a DirLike<'a>>,
    meta: Metadata,
    path: String,
    indent: i32,
}
impl Listable for Leaf {
    fn draw(&self, sdl: &mut SdlContainer, font: &Font, pos: (i32, i32)) -> i32 {
        sdl.draw_txt(&self.name, (self.indent*40+20, pos.1), &font);
        pos.1 + 20
    }
}


struct Thumbnailable {
    name: String,
    //parrent: Option<&'a DirLike<'a>>,
    meta: Metadata,
    //thumbnail: 
    indent: i32,
}


struct SdlContainer {
    canvas: sdl2::render::WindowCanvas,
    context: sdl2::Sdl,
    event_sender: sdl2::event::EventSender,
}
impl SdlContainer {
    fn draw_txt(&mut self, txt: &str, pos: (i32, i32), font: &Font ) {
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

struct BumpEvent {
}

fn draw_statusbar() {

}


fn main() {
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
    let window = video_subsystem.window("LumberJack /home/tesztenv/", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(20, 20, 20));
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

    let root_path = "/home/tesztenv";
    let mut root_node = FSnode::DirLike(DirLike {name: String::from(""),
                                                path: String::from(root_path),
                                                indent: -1,
                                                meta: fs::metadata(root_path).unwrap(),
                                                opened: false,
    });
    let mut list_view: Vec<Box<FSnode>> = Vec::new();
    root_node.open(&mut list_view, 0);
    dbg!(list_view.len());
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
                unsafe {
                    let node = &mut list_view[cursorpos] as *mut Box<FSnode>;
                    let node = &mut *node;
                    node.open(&mut list_view, cursorpos);
                }
            }
            Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                unsafe {
                    let node = &mut list_view[cursorpos] as *mut Box<FSnode>;
                    let node = &mut *node;
                    node.close(&mut list_view, cursorpos);
                }
            }

            _ => {dbg!("Other event!", event);}
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
            if i + viewpos == cursorpos { //TODO: Holly shit as i32
                sdl.canvas.set_draw_color(cursorcolor);
            } else if iseven {
                sdl.canvas.set_draw_color(bg2);
            } else {
                sdl.canvas.set_draw_color(bg1);
            }
            iseven = !iseven;
            sdl.canvas.fill_rect(Some(Rect::new(0,pos,4000,4000))).expect("Jaj!");
            pos = entry.draw(&mut sdl, &font, (0, pos));
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

