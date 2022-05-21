
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
use std::rc::{Rc,Weak};

use std::cell::RefCell;


enum FSnode {
    DirLike(DirLike),
    Leaf(Leaf),
}
impl FSnode {
    fn draw(&self, sdl: &mut SdlContainer, font:&Font, pos: (i32, i32)) -> i32 {
        let name = match self {
            Self::DirLike(d) => {&d.name},
            Self::Leaf(f) => {&f.name},
        };
        sdl.draw_txt(name, pos, &font);
        pos.1 + 20
    }
    fn open(&mut self, list_view: &mut Vec<FSnode>, pos: usize) {
        match self {
            Self::DirLike(d) => {d.open(list_view, pos)},
            Self::Leaf(f) => {println!("File Open")}, // TODO implement XDG open
        }
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
    fn open(&mut self, list_view: &mut Vec<FSnode>, pos: usize) {
        for file in fs::read_dir(&self.path).unwrap() {
            let file = file.unwrap();
            if file.file_type().unwrap().is_dir() {
                list_view.insert(pos+1, FSnode::DirLike(DirLike{
                    name: file.file_name().into_string().unwrap(),
                    path: file.path().into_os_string().into_string().unwrap(),
                    meta: file.metadata().unwrap(),
                    //parrent: Some(& self),
                    //children: Vec::new(),
                    opened: false,
                    indent: self.indent+1,
                }));
            }
            else {
                list_view.insert(pos+1, FSnode::Leaf(Leaf{
                    name: file.file_name().into_string().unwrap(),
                    path: file.path().into_os_string().into_string().unwrap(),
                    meta: file.metadata().unwrap(),
                    indent: self.indent+1,
                }));
            };
        }
        self.opened = true;

    }
}


struct Leaf {
    name: String,
    //parrent: Option<&'a DirLike<'a>>,
    meta: Metadata,
    path: String,
    indent: i32,
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


fn main() {
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
    let mut sdl = SdlContainer {
        canvas: canvas,
        context: sdl_context,
    };
    let mut event_pump = sdl.context.event_pump().unwrap();

    let root_path = "/";
    let mut root_node = FSnode::DirLike(DirLike {name: String::from(""),
                                                path: String::from(root_path),
                                                indent: -1,
                                                meta: fs::metadata(root_path).unwrap(),
                                                opened: true,
    });
    let mut list_view: Vec<FSnode> = Vec::new();
    root_node.open(&mut list_view, 0);

    let cursorpos = 0;
    let selection: Vec<isize>;

    'main_loop : loop {
        let event = event_pump.wait_event();
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                break 'main_loop
            },
            _ => {}
        }
        sdl.canvas.clear();
        let mut pos = 0;
        for entry in &list_view {
            pos = entry.draw(&mut sdl, &font, (0, pos));
        }
    }
}

