
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


trait Manipulate {
    fn close(&mut self, list_view: &mut Vec<Box<FSnode>>, pos: usize) {}
}


trait Listable {
    fn draw(&self, sdl: &mut SdlContainer, font:&Font, pos: (i32, i32)) -> i32 ;
}



enum FSnode {
    DirLike(DirLike),
    Leaf(Leaf),
}
impl FSnode {
    fn open(&mut self, list_view: &mut Vec<Box<FSnode>>, pos: usize) {
        match self {
            Self::DirLike(d) => {d.open(list_view, pos)},
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
            Self::DirLike(d) => {d.close(list_view, pos);},
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
}
impl Listable for DirLike {
    fn draw(&self, sdl: &mut SdlContainer, font: &Font, pos: (i32, i32)) -> i32 {
        sdl.draw_txt(&self.name, (self.indent*40, pos.1), &font);
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
        sdl.draw_txt(&self.name, (self.indent*40, pos.1), &font);
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
    let mut sdl = SdlContainer {
        canvas: canvas,
        context: sdl_context,
    };
    let mut event_pump = sdl.context.event_pump().unwrap();

    let root_path = "/home/tesztenv";
    let mut root_node = FSnode::DirLike(DirLike {name: String::from(""),
                                                path: String::from(root_path),
                                                indent: -1,
                                                meta: fs::metadata(root_path).unwrap(),
                                                opened: true,
    });
    let mut list_view: Vec<Box<FSnode>> = Vec::new();
    root_node.open(&mut list_view, 0);

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

            _ => {}
        }
        sdl.canvas.clear();
        let mut pos = 0;
        let mut iseven = false;
        for (i, entry) in list_view[viewpos..].iter().enumerate() {
            if i - viewpos ==cursorpos {
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
    }
}

