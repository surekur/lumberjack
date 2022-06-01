use crate::FSnode;


enum Mod {
    Normal(Normal),
    Visual(Visual),
}


struct Normal {
    name: String,
    commandsequence: Vec<Command>,

}



struct Rebane {

}
impl Normal {
    fn parsecommandsequance(commands: &str) -> Vec<Command> {
        vec![]
    }

}

struct Visual {

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

// NORMAL:
// fullsequencies :
// targ
// quant targ
// manip targ
// manip <manip>
// quant manip


// NORMAL:
// fullsequencies :
