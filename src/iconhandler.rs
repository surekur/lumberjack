use freedesktop_icons::lookup;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;


pub struct Icons {
    theme: String,
    //strorage: HashMap<String, >
}
impl Icons {
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
            .unwrap();
        Self {
            theme: theme,
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
        ).unwrap()
    }

    pub fn get_icon(&mut self, file: &Path) -> PathBuf {
        let mime = self.get_mime(&file);
        lookup(&mime)
            .with_size(16)
            .with_scale(1)
            .with_theme(&self.theme)
            .with_cache()
            .find()
            .unwrap()
    }   
}

