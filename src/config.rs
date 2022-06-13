use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use crate::commands;
use crate::modality::Command;

pub static LINECOUNTERWIDTH: u32 = 30;
pub static LINECOUNTER_BG0: Color = Color::RGB(40, 40, 40);
pub static LINECOUNTER_BG1: Color = Color::RGB(50, 50, 50);
pub static LINECOUNTER_FG: Color = Color::RGB(140, 140, 140);
pub static STATUS_BG: Color = Color::RGB(60, 60, 60);

pub static CURSOR_COLOR: Color = Color::RGB(0, 0, 140);
pub static CURSOR_ON_LINECOUNTER: Color = Color::RGB(30, 30, 100);
pub static MINIMAP_BG: Color = Color::RGB(50, 50, 50);
pub static MINIMAP_FG: Color = Color::RGB(200, 200, 200);
pub static MINIMAP_FG_VIEWPORT: Color = Color::RGB(255, 255, 255);
//pub static MINIMAP_FG_VIEWPORT: Color = Color::RGB(40, 160, 40);
pub static BG1: Color = Color::RGB(20,20,20);
pub static BG0: Color = Color::RGB(25,25,25);
pub static MOUSE_HOOVER: Color = Color::RGB(30,30,60);
pub static FG_GENERAL: Color = Color::RGB(255, 255, 255);

pub static COM_MANIP_COLOR: Color = Color::RGB(200, 200, 45);
pub static COM_TARGET_COLOR: Color = Color::RGB(125, 125, 200);
pub static COM_TARGETLESS_COLOR: Color = Color::RGB(125, 200, 125);
pub static COM_QUANTIFYER_COLOR: Color = Color::RGB(200, 125, 200);



macro_rules! keytocommand {
    ( $key:expr ) => {
        match key {
            Keycode::Y => {
                Command::Manip(commands::yankt)
            }
            Keycode::P => {
                Command::Targetless(commands::past)
            }
        }
    }
}


pub type TextureCreator = sdl2::render::TextureCreator<sdl2::video::WindowContext>;

