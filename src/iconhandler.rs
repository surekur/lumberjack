use freedesktop_icons::lookup;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use xdg_mime::SharedMimeInfo;
use std::fs::Metadata;
use sdl2::render::Texture;
use crate::render::SdlContainer;
use sdl2::image::{InitFlag, LoadTexture};
use std::collections::HashMap;

pub type TexCre = sdl2::render::TextureCreator<sdl2::video::WindowContext>;

//#[derive(Debug)]
pub struct Icons<'w> {
    pub theme: String,
    mimedb: SharedMimeInfo,
    pub loaded: Vec<Texture<'w>>,
    //texturecreator: TexCre,
    mimesloaded: HashMap<String, usize>,
    //strorage: HashMap<String, >
}
impl<'w> Icons<'w> {
    pub fn new() -> Self { //TODO: other systems 
        let theme = Command::new("xfconf-query")
            .arg("-c")
            .arg("xsettings")
            .arg("-p")
            .arg("/Net/IconThemeName")
            .output()
            .expect("Cant find the icon theme")
            .stdout;
        let theme = String::from_utf8(theme)
            .expect("Nem utf???")
            .trim()
            .to_owned();
        Self {
            theme: theme,
            mimedb: SharedMimeInfo::new(),
            loaded: Vec::new(),
            mimesloaded: HashMap::new()
            //texturecreator: sdl.canvas.get_
        }
    }

    fn get_mime(&self, file: &Path) -> String { // TODO probably it is better 
                                               //to ask once for the whole dir.
        String::from_utf8(
        Command::new("file")
            .arg("--mime")
            .arg(&file)
            .output()
            .expect("Failed to execute 'file' command. ")
            .stdout
        ).expect("Nem utf?")
        .trim()
        .to_owned()
    }

    pub fn get_icon(&mut self, path: &Path, meta: &Metadata, tc: &'w TexCre) -> usize {
        let guess = self.mimedb.guess_mime_type()
            .path(path)
            //.metadata(meta)
            .guess();
        let mime = guess.mime_type();
        let mimesloaded = &mut self.mimesloaded as *mut HashMap<String, usize>; 
        let mimesloaded = unsafe {&mut *mimesloaded};
        let iconid = mimesloaded.entry(mime.to_string())
            .or_insert({
            
            let icon = self.mimedb.lookup_icon_names(guess.mime_type());
            //(&icon[0]).to_owned()
            let icon  = lookup(&icon[0])
                 .with_size(16)
                 .with_scale(1)
                 .with_theme(&self.theme)
                 .with_cache()
                 .find();
            match icon {
                Some(icon) => {self.load(icon, tc)},
                None => {1} // TODO: Preload a generic file icon!
            }
            
            });
        *iconid
    }


    fn load(&mut self, filename:  PathBuf, tc: &'w TexCre) -> usize {
        let texture = tc.load_texture(&filename)
            .expect("Can't load icon.");
        self.loaded.push(texture);
        println!("Icon loaded: {:?}", &filename);
        self.loaded.len() - 1
    }
}

