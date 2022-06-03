use crate::FSnode;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::fsnodetypes::{ListView, Manipulate};
use crate::iconhandler::Icons;
use crate::TextureCreator;

trait ActMod {


}

pub enum Mode {
    Normal(NormalMode),
    Visual(VisualMode),
    Rebane(RenameMode),
    Command(Command),
}


pub struct NormalMode {
    name: String,
    commandsequence: Vec<Command>,

}
impl NormalMode {
    pub fn new() -> Self {
        Self {
            name: String::from("--NORMAL--"),
            commandsequence: Vec::new(),
            
        }


    }
    fn parsecommandsequance(commands: &str) -> Vec<Command> {
        vec![]
    }

    pub fn handle_input<'a>(&mut self, event: Event, cursorpos: &mut usize, list_view: &mut ListView, icons: &mut Icons<'a>,
                    texturecreator: &'a TextureCreator) -> bool {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return false;
            },
            Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                if *cursorpos > 0 {
                    *cursorpos -= 1;
                }
            }
            Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                if *cursorpos < list_view.len()-1 {
                    *cursorpos += 1;
                }
            }
            Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                let node = list_view[*cursorpos].clone();
                let mut node = node.borrow_mut();
                node.open(list_view, *cursorpos, &texturecreator , icons);
            }
            Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                let node = list_view[*cursorpos].clone();
                let mut node = node.borrow_mut();
                node.close(list_view, *cursorpos);
            }
            Event::KeyDown { keycode: Some(Keycode::Return), ..} => {
            }

            _ => {}//dbg!("Other event!", event);}
        }
    return true;

    }

}



pub struct RenameMode {

}


pub struct CommandMode{

}



pub struct VisualMode {

}



enum Command {
    Target(fn (list_view: &Vec<FSnode>, cursorpos: usize) -> (usize, usize)),
    Quantifyer(usize),
    Manip(),
    Targetless,

}
    

enum Manip {
    Yankt,
    Past,
    SoftDel,
    Del,
    Rename,
    RenameAppend,
}

enum Target {
    Up,
    Down,
    First,
    Last,
    FirstInParrent,
    LastOfParrent,
    Parrent,




}

enum Targetless {


}

// NORMAL:
// fullsequencies :
// tless
// quant tless
// targ
// quant targ
// manip targ
// manip <manip>
// quant manip


// VISUAL:
// fullsequencies :
//
//
