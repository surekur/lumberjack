use sdl2::ttf::Font;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;

use std::os::unix;
use std::path::Path;
use std::fs;

struct SdlContainer<'a> {
    canvas: sdl2::render::WindowCanvas,
    context: sdl2::Sdl,
    ttf: TtfContainer<'a>,
    
}
impl SdlContainer<'_> {
    fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let ttf_context: sdl2::ttf::Sdl2TtfContext = sdl2::ttf::init()
            .unwrap();
        let font = ttf_context.load_font(
            Path::new("/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf"),
            15).unwrap();
        //dbg!(ttf_context);
        let window = video_subsystem.window("LumberJack /home/tesztenv/", 800, 600)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        
        Self {
            canvas: canvas,
            context: sdl_context,
            ttf: TtfContainer {
                font: font,
                context: ttf_context,
            }
        }
    }
}

struct TtfContainer<'a> {
    font: Font<'a, 'a>,
    context: sdl2::ttf::Sdl2TtfContext,
}
impl TtfContainer<'_> {

    fn draw_txt(&mut self, txt: &str, pos: (i32, i32), &mut canvas: &mut WindowCanvas) {
        let surf = self.font.render(txt)
            .blended(Color::RGB(255,255,255))
            .unwrap();
        let texture_creator = canvas.texture_creator();
        let texture = sdl2::render::Texture::from_surface(&surf, &texture_creator)
            .unwrap();
        let size = surf.size();
        canvas.copy(&texture, None, Rect::new(pos.0, pos.1, size.0, size.1))
            .expect("yay thats a bug in draw_txt(), yay!");
    }
}

struct FSnode {
    name: String,
    parrent: Option<Box<FSnode>>,
    //is_dir : bool,
    dir_like : bool,
    level: i32,
    meta: fs::Metadata,
}


struct LumberJack<'a> {
    list_view: Vec<FSnode>,
    cursorpos: i64,
    sdl: SdlContainer<'a>,
  //  texture_creator: , //sdl::render::
}

impl LumberJack<'_> {
    fn run(&mut self) {
        let mut event_pump = self.sdl.context.event_pump().unwrap();
        self.open_dir("/home/tesztenv");


        //let mut iter = 0;
        'running : loop {
            //iter+=1;
            self.sdl.canvas.clear();
            self.test_draw();
            self.sdl.canvas.present();
            let event = event_pump.wait_event();
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
            //dbg!(("runs!", iter));
            let ev =event.is_keyboard();
            //dbg!(ev);
            //std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
        }

    }
    fn test_draw(&mut self) {
        self.sdl.ttf.draw_txt("Yay! It's werks!", (20,20), &mut self.sdl.canvas);
        //canvas.texture_creator()
    }

    
    fn open_dir(&mut self, dir: &str) {
        for file in fs::read_dir(dir).unwrap() {
            let file = file.unwrap();
            self.list_view.push(FSnode {
                name : file.file_name().into_string().unwrap(),
                meta : file.metadata().unwrap(),
                dir_like: false,
                //is_dir: false,
                parrent: None,
                level  :0,
                
            });
        };
    }
    fn test_open_dir(&mut self, dir: &str, lvl: i32) {

        for file in fs::read_dir(dir).unwrap() {
            let file = file.unwrap();
            if file.file_type().unwrap().is_dir() {
                
            }
            self.list_view.push(FSnode {
                name : file.file_name().into_string().unwrap(),
                meta : file.metadata().unwrap(),
                dir_like: false,
                //is_dir: false,
                parrent: None,
                level  :lvl,
                
            });
        };
    }

    fn draw_listview(&mut self) {
        let mut i = 0;
       // let mut canvas = &self.sdl.canvas;
        for line in &self.list_view {
            //let line = line;
            i += 1;
            self.sdl.ttf.draw_txt(&line.name, (50*line.level, i*20), &mut self.sdl.canvas);
        }
    }
}

fn main() {
   // let texture_creator = canvas.texture_creator();
    let mut lumberjack = LumberJack {
        sdl: SdlContainer::new(),
        cursorpos: 0,
        list_view: vec![],
       // texture_creator: texture_creator,
    };
    lumberjack.run();

}

