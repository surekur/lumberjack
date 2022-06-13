use crate::config::*;
use crate::iconhandler::Icons;
use crate::ListView;
use crate::fsnodetypes::{FSnode, Leaf, DirLike, Thumbnailable};
use crate::modality::{Command, Mode};
use sdl2::rect::Rect;
use sdl2::ttf::Font;
use sdl2::pixels::Color;
use sdl2::gfx::primitives::DrawRenderer;

pub struct SdlContainer {
   pub canvas: sdl2::render::WindowCanvas,
   pub context: sdl2::Sdl,
   pub event_sender: sdl2::event::EventSender,
}
impl SdlContainer {
    pub fn draw_txt(&mut self, txt: &str, pos: (i32, i32), font: &Font, color: Color) {
        let surf = font.render(txt)
            .blended(color)
            .unwrap();
        let texture_creator = self.canvas.texture_creator();
        let texture = sdl2::render::Texture::from_surface(&surf, &texture_creator)
            .unwrap();
        let size = surf.size();
        self.canvas.copy(&texture, None, Rect::new(pos.0, pos.1, size.0, size.1)).ok();
    }
}

pub fn draw_linecounter(height: u32, number: usize, pos: i32, 
                    sdl: &mut SdlContainer, font: &Font, iseven: bool) {
    let rect = Rect::new(0, pos, LINECOUNTERWIDTH, height);
    if number == 0 {
        sdl.canvas.set_draw_color(CURSOR_ON_LINECOUNTER);
        sdl.canvas.fill_rect(rect).ok();
        return}
    if iseven {
        sdl.canvas.set_draw_color(LINECOUNTER_BG0);
    }
    else {
        sdl.canvas.set_draw_color(LINECOUNTER_BG1);
    }
    sdl.canvas.fill_rect(rect).ok();
    let text = number.to_string();
    let posx = LINECOUNTERWIDTH - font.size_of(&text[..]).unwrap().0 - 6;
    sdl.draw_txt(&text[..], (posx as i32, pos), font, LINECOUNTER_FG);
}

pub fn draw_minimap(list_view: & ListView, viewpos: usize, lastvisible: usize,
                sdl: &mut SdlContainer, height: u32, width: u32) {
    let winsize = sdl.canvas.window().drawable_size();
    let xpos = winsize.0 as i32 - width as i32;
    sdl.canvas.set_draw_color(MINIMAP_BG);
    //sdl.canvas.fill_rect(Rect::new(xpos as i32, 0, width, height));
    // TODO: Transparent bg?
    let mut pos: i32 = 2;
    for (i, entry) in list_view.iter().enumerate() {
        if viewpos <=  i && i < lastvisible {
            sdl.canvas.set_draw_color(MINIMAP_FG_VIEWPORT);
        }
        else {
            sdl.canvas.set_draw_color(MINIMAP_FG);
        }
        let entry = entry.borrow_mut();
        let (indent, entrheight, len) = match &*entry {
            FSnode::DirLike(f) => {(f.indent, f.get_height(), f.name.len())},
            FSnode::Leaf(f) => {(f.indent, f.get_height(), f.name.len())},
        };
        let entrheight = entrheight as u32 / 10;
        sdl.canvas.fill_rect(Rect::new(xpos+2+(indent*4), pos, len as u32, entrheight)).ok();
        pos += entrheight as i32 + 1; 
    }
}

pub fn draw_statusbar(top: i32, cursorpos: usize, list_view: &mut ListView,
                  mode: &mut Mode, sdl: &mut SdlContainer, font: &Font) {
    let winsize = sdl.canvas.window().drawable_size();
    let contenttop = winsize.1 as i32 - 20;
    let height = winsize.1 - top as u32;

    sdl.canvas.set_draw_color(STATUS_BG);
    sdl.canvas.fill_rect(Rect::new(0, top, winsize.0, winsize.1-top as u32)).ok();
    let mut pos = 0;

    sdl.canvas.set_draw_color(mode.color);
    let modewidth = font.size_of(&mode.name[..]).unwrap().0+60;
    let rect = Rect::new(0, top, modewidth, height);
    sdl.canvas.fill_rect(rect).ok();
    sdl.draw_txt(&mode.name[..], (30, contenttop), font, mode.fgcolor);
    sdl.draw_txt(&format!("total:{}", list_view.len())[..],
    ((winsize.0 - 100) as i32, contenttop), font, FG_GENERAL);
    pos += modewidth as i32 ;
    let mut commandseq = &mode.commandseq;
    let mut executed = false;
    let mut previuscolor = mode.color;
    if commandseq.is_empty() {
        commandseq = &mode.executedseq;
        executed = true;
    }
    else { 
        println!("ELSE!");
        dbg!(pos, top, winsize.1, height);
        sdl.canvas.filled_trigon(
            pos as i16, top as i16,  (pos+10) as i16, (top+(height as i32/2)) as i16,
            pos as i16, winsize.1 as i16,
            Color::RGB(255,255,255)
        ).ok();
    }
    for command in commandseq[..].iter() {
        let mut tmpstr = String::new();
        let (color, text) = {
            match command {
                Command::Manip{fun: _, name: n} => {(COM_MANIP_COLOR, n)},
                Command::Target{fun: _, name: n} => {(COM_TARGET_COLOR, n)},
                Command::Quantifyer(q) => {(COM_QUANTIFYER_COLOR, {tmpstr = format!("{}", q);
                                                                    &tmpstr})},
                Command::Targetless{fun: _, name: n} => {(COM_TARGETLESS_COLOR, n)},
            }
        };
        let size = font.size_of(&text).unwrap().0+20;
        sdl.canvas.set_draw_color(color);
        sdl.canvas.fill_rect(Rect::new(pos, top, size, height)).ok();
        if !executed {
            sdl.canvas.filled_trigon(
                pos as i16, top as i16,  (pos+5) as i16, (top+(height as i32/2)) as i16,
                pos as i16, winsize.1 as i16,
                previuscolor
            ).ok();
        }
        sdl.draw_txt(&text, (pos+10, contenttop), font, BG0);
        pos += size as i32;
        if !executed {
            previuscolor = color;
            sdl.canvas.filled_trigon(
                pos as i16, top as i16,  (pos+5) as i16, (top+(height as i32/2)) as i16,
                pos as i16, winsize.1 as i16,
                color
            ).ok();
        }
    }
}


pub trait Listable {
    fn draw(&self, sdl: &mut SdlContainer, font: &Font, pos: (i32, i32), icons: &Icons) -> i32 ;
    fn get_height(&self) -> i32 {20}
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


impl Listable for DirLike {
    fn draw(&self, sdl: &mut SdlContainer, font: &Font, pos: (i32, i32), icons: &Icons) -> i32 {
        sdl.canvas.copy(&icons.loaded[self.icon], None, 
                        Rect::new(self.indent * 40 + 22+ pos.0, pos.1 + 2, 16, 16)).ok();
        let gpos = ((pos.0 as i16) + (self.indent as i16 * 40), pos.1 as i16);
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
        sdl.draw_txt(&self.name, (self.indent  *40+pos.0+40, pos.1), &font, FG_GENERAL);
        pos.1 + 20
    }
}


impl Listable for Leaf {
    fn draw(&self, sdl: &mut SdlContainer, font: &Font, pos: (i32, i32), icons: &Icons) -> i32 {
        sdl.canvas.copy(&icons.loaded[self.icon], None, 
                        Rect::new(self.indent as i32 *40+22+pos.0, pos.1+2, 16, 16)).ok();
        sdl.draw_txt(&self.name, (self.indent as i32 *40+40+pos.0, pos.1), &font, FG_GENERAL);
        pos.1 + 20
    }
}

