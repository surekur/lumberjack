
// TODO: should be moved to icon lookup src file
//
// rust implementation of free-desktop icon lookup



fn find_icon(icon: &str, size: i16, scale: i8) -> Path {
    
}

fn find_icon_helper(icon: &str, size: i16, scale: i8, theme: &str) -> Option<Path> {

}

// helpers

fn look_up_icon(iconname, size: i16, scale: i8, theme: &str) -> Option<Path> {

}
    

fn icon_basedirs() -> [&str; 3] {
    let home = env::var("HOME").unwrap();
    let xdgdata = env::var("XDG_DATA_DIRS").unwrap();
    
    [home+"/.icons", xdgdata+"/icons", "/usr/share/pixmaps"]
}



