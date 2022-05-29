use freedesktop_icons::lookup;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use xdg_mime::SharedMimeInfo;
use std::fs::Metadata;
use crate::fsnodetypes::SdlContainer;
use sdl2::render::Texture;
use sdl2::image::{InitFlag, LoadTexture};

pub struct Icons<'a> {
    pub theme: String,
    mimedb: SharedMimeInfo,
    loaded: Vec<Texture<'a>>,
    //strorage: HashMap<String, >
}
impl Icons<'_> {
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

    pub fn get_icon(&mut self, path: &Path, meta: &Metadata, sdl: &SdlContainer) -> isize {
        let guess = self.mimedb.guess_mime_type()
            .path(path)
            //.metadata(meta)
            .guess();
        dbg!(&guess.mime_type());
        let icon = self.mimedb.lookup_icon_names(guess.mime_type());
        dbg!(&icon);
        //(&icon[0]).to_owned()
        let icon  = lookup(&icon[0])
             .with_size(16)
             .with_scale(1)
             .with_theme(&self.theme)
             .with_cache()
             .find()
             .expect("Can't find icon!");
        dbg!(&icon);
        3
    }


    fn load<'a>(&'a mut self, file:  PathBuf, sdl: &'a SdlContainer) -> isize {
        let texture: Texture<'a> = sdl.texturecreator.load_texture(file)
            .expect("Can't load icon.");
        self.loaded.push(texture);
        3
    }
}

