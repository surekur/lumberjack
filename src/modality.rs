use crate::FSnode;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use crate::fsnodetypes::{ListView, Manipulate};
use crate::iconhandler::Icons;
use crate::TextureCreator;
use crate::config::*;
use crate::commands;
use crate::commands::{ManipFun, TargetFun, TargetlessFun};
use std::path::PathBuf;
use crate::GlobalState;

pub type CommandSeq = Vec<Command>;
//pub type ModeInputHandler = 

pub struct Mode<'a> {
    pub name: String,
    pub color: Color,
    pub fgcolor: Color,
    pub keysequence: Vec<Keycode>,
    pub commandseq: Vec<Command>,
    pub executedseq: Vec<Command>,
    pub handle_input: fn(&mut Self , Event, glob:  &mut GlobalState,
                         &mut ListView, &mut Icons<'a>, &'a TextureCreator),
}
impl Mode<'_> {
    pub fn normal() -> Self {
        Self {
            name: String::from("NORMAL"),
            color: Color::GREY,
            fgcolor: BG0,
            keysequence: Vec::new(),// TODO Made them arrays instead of vecs(?)
            commandseq: Vec::new(), // or arrays with size
            executedseq: Vec::new(),
            handle_input: Mode::NORMAL_handle_input,
        }
    }

    fn parse_commandsequance(commands: &Vec<Keycode>) -> Option<Vec<Command>> {
        let mut commandseq = Vec::new();
        let mut previusnum = String::new();
        for key in commands {
            let command = match key {
                // TODO: thats kinda dump, I know...
                // Should be replaced with something
                // less naïve!
                Keycode::Num0 => {
                    if !previusnum.is_empty() {
                        commandseq.pop();
                    }
                    previusnum.push('0');
                    Command::Quantifyer(previusnum.parse().unwrap())
                }
                Keycode::Num1 => {
                    if !previusnum.is_empty() {
                        commandseq.pop();
                    }
                    previusnum.push('1');
                    Command::Quantifyer(previusnum.parse().unwrap())
                }
                Keycode::Num2 => {
                    if !previusnum.is_empty() {
                        commandseq.pop();
                    }
                    previusnum.push('2');
                    Command::Quantifyer(previusnum.parse().unwrap())
                }
                Keycode::Num3 => {
                    if !previusnum.is_empty() {
                        commandseq.pop();
                    }
                    previusnum.push('3');
                    Command::Quantifyer(previusnum.parse().unwrap())
                }
                Keycode::Num4 => {
                    if !previusnum.is_empty() {
                        commandseq.pop();
                    }
                    previusnum.push('4');
                    Command::Quantifyer(previusnum.parse().unwrap())
                }
                Keycode::Num5 => {
                    if !previusnum.is_empty() {
                        commandseq.pop();
                    }
                    previusnum.push('5');
                    Command::Quantifyer(previusnum.parse().unwrap())
                }
                Keycode::Num6 => {
                    if !previusnum.is_empty() {
                        commandseq.pop();
                    }
                    previusnum.push('6');
                    Command::Quantifyer(previusnum.parse().unwrap())
                }
                Keycode::Num7 => {
                    if !previusnum.is_empty() {
                        commandseq.pop();
                    }
                    previusnum.push('7');
                    Command::Quantifyer(previusnum.parse().unwrap())
                }
                Keycode::Num8 => {
                    if !previusnum.is_empty() {
                        commandseq.pop();
                    }
                    previusnum.push('8');
                    Command::Quantifyer(previusnum.parse().unwrap())
                }
                Keycode::Num9 => {
                    if !previusnum.is_empty() {
                        commandseq.pop();
                    }
                    previusnum.push('9');
                    Command::Quantifyer(previusnum.parse().unwrap())
                }
                Keycode::Y => {
                    previusnum.clear();
                    Command::Manip{fun: commands::yank, name: String::from("yank")}
                }
                Keycode::P => {
                    previusnum.clear();
                    Command::Targetless{fun: commands::past, 
                                    name: String::from("past")}
                }
                
                Keycode::Up => {
                    previusnum.clear();
                    Command::Target{fun: commands::up, name: String::from("↑")}
                }
                Keycode::Down => {
                    previusnum.clear();
                    println!("DOWN");
                    Command::Target{fun: commands::down, name: String::from("↓")}
                }
                _ => {return None}
            };
            commandseq.push(command);
        }
        Some(commandseq)
    }

    fn NORMAL_exec_command_seq(&self, mut list_view: &mut ListView,
                            glob:  &mut GlobalState) -> CommandSeqState {
        match self.commandseq[..] {
            // Complette:
            // TG
            // Qn TG
            // Mn Tg
            // Mn Qn Tg
            // Mn Mn
            // Tl
            // Qn Tl
            [Command::Target{name: _, fun: tg}] => {
                match tg(list_view, glob.cursorpos) {
                    Some(t) => {glob.cursorpos = t;},
                    None => {}
                }
                CommandSeqState::Complette
            },

            [Command::Quantifyer(qf),
            Command::Target{name: _, fun: tg}] => {
                let mut target = glob.cursorpos;
                for _ in 0..qf {
                    if let Some(t) = tg(list_view, target) {
                        target = t;
                    }
                }
                glob.cursorpos = target;
                CommandSeqState::Complette
            },

            [Command::Manip{name: _, fun: mn},
            Command::Target{name: _, fun: tg}] => {
                let mut target = glob.cursorpos;
                if let Some(t) = tg(list_view, glob.cursorpos) {
                    target = t;
                }
                let (start, end) =
                    if glob.cursorpos < target {(glob.cursorpos, target)}
                    else                    {(target, glob.cursorpos)};
               // for i in start..=end {
               //     mn();
               // }
                mn((start..=end).collect() , &mut list_view);
                CommandSeqState::Complette
            },

            [Command::Manip{name: _, fun: mn},
            Command::Quantifyer(qf),
            Command::Target{name: _, fun: tg}] => {
                let mut target = glob.cursorpos;
                for _ in 0..qf {
                    if let Some(t) = tg(list_view, target) {
                        target = t;
                    }
                }
                let (start, end) =
                    if glob.cursorpos < target {(glob.cursorpos, target)}
                    else                    {(target, glob.cursorpos)};
                mn((start..=end).collect() , &mut list_view);
                CommandSeqState::Complette
            },

            [Command::Manip{name: _, fun: mn},
            Command::Manip{name: _, fun: mn2}]
                    if mn as *const ManipFun == mn2 as *const ManipFun => {
                        mn(vec![glob.cursorpos], &mut list_view); 
                        CommandSeqState::Complette
                },
            
            [Command::Targetless{name: _, fun: tl}] => {
                tl(&mut list_view, glob.cursorpos);
                CommandSeqState::Complette
            },

            [Command::Quantifyer(qf),
            Command::Targetless{name: _, fun: tl}] => {
                for _ in 0..=qf {
                    tl(&mut list_view, glob.cursorpos);
                }
                CommandSeqState::Complette
            },

            // Uncomplette:
            [Command::Manip{name: _, fun: _}] => {CommandSeqState::Uncomplette
            },

            [Command::Manip{name: _, fun: _},
            Command::Quantifyer(_)] => {CommandSeqState::Uncomplette
            },

            [Command::Quantifyer(_)] => {CommandSeqState::Uncomplette
            },

            _ => {
                CommandSeqState::Broken
            }

        }
        
    }


    pub fn NORMAL_handle_input<'a>(&mut self, event: Event, glob:  &mut GlobalState,
                                   list_view: &mut ListView, icons: &mut Icons<'a>,
                                   texturecreator: &'a TextureCreator) {
        match event {
 //           Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
 //               if glob.cursorpos > 0 {
 //                   glob.cursorpos -= 1;
 //               }
 //           }
 //           Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
 //               if glob.cursorpos < list_view.len()-1 {
 //                   glob.cursorpos += 1;
 //               }
 //           }
            Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                let node = list_view[glob.cursorpos].clone();
                {let mut node = node.borrow_mut();
                node.open(list_view, glob.cursorpos, &texturecreator , icons);}
            }
            Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                let node = list_view[glob.cursorpos].clone();
                {let mut node = node.borrow_mut();
                node.close(list_view, glob.cursorpos);}
            }
            Event::KeyDown { keycode: Some(Keycode::Return), ..} => {
            }

            Event::KeyDown{ keycode: Some(k), ..} => {
                self.keysequence.push(k);
                if let Some(cseq) = Self::parse_commandsequance(&self.keysequence) {
                    self.commandseq = cseq;
                }
                else {self.commandseq.clear();}
                let comseqstate = self.NORMAL_exec_command_seq(list_view, glob);
                match comseqstate {
                    CommandSeqState::Complette => {
                        unsafe {
                            std::ptr::swap( &mut self.executedseq as *mut CommandSeq,
                                            &mut self.commandseq as *mut CommandSeq);
                        }
                        self.keysequence.clear();
                        self.commandseq.clear();
                    },
                    CommandSeqState::Broken => {
                        self.keysequence.clear();
                        self.commandseq.clear();
                    }
                    CommandSeqState::Uncomplette => {}
                }
            }

            _ => {}//dbg!("Other event!", event);}
        }

    }

}


pub fn precommand_highlight(commandseq: Vec<Command>, list_view: ListView, pos: usize) {

}

pub enum Command {
    Target{ fun: TargetFun, name: String},
    Quantifyer(usize),
    Manip{fun: ManipFun, name: String}, //returns the lv location of manipulated directories.
    Targetless{fun: TargetlessFun, name: String} // -||-
}
use std::fmt;
impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (comtype, cont) = match self {
            Self::Target{fun: _, name: n} => {("Targ", n.clone())},
            Self::Targetless{fun: _, name: n} => {("Tless", n.clone())},
            Self::Manip{fun: _, name: n} => {("Manip", n.clone())},
            Self::Quantifyer(q) => {("Quant", format!("{}",q))},
        };
        f.debug_struct(comtype) 
            .field("fun", &cont)
            .finish()
    }
}


enum CommandSeqState {
    Complette,
    Uncomplette,
    Broken,
}

//enum Manip {
//    Yankt,
//    SoftDel,
//    Del,
//    Rename,
//    RenameAppend,
//}
//
//enum Target {
//    Up,
//    Down,
//    First,
//    Last,
//    FirstInParrent,
//    LastInParrent,
//    Parrent,
//}
//
//enum Targetless {
//    Past,



//pub enum Mode {
//    Normal(NormalMode),
//    Visual(VisualMode),
//    Rebane(RenameMode),
//    Command(Command),
//}

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
