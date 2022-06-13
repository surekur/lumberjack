use crate::fsnodetypes::ListView;


pub type ManipFun = 
fn (files: Vec<usize>, list_view: &mut ListView ) -> Vec<usize>;
pub type TargetFun =
fn (list_view: &ListView, cursorpos: usize) -> Option<usize>;
pub type TargetlessFun =
fn (list_view: &mut ListView, pos: usize);

// Manip
pub fn yank(files: Vec<usize>, list_view: &mut ListView ) -> Vec<usize> {
    println!("YANK!!! {:?}", files);
    Vec::new()

}

// Target
pub fn up(list_view: &ListView, cursorpos: usize) -> Option<usize> {
    if cursorpos > 0 {Some(cursorpos-1)}
    else {None}
}


pub fn down(list_view: &ListView, cursorpos: usize) -> Option<usize> {
    if cursorpos < list_view.len() - 1 {Some(cursorpos+1)}
    else {None}
    
}

// Targetless
pub fn past(list_view: &mut ListView, pos: usize ) {
    println!("PAST!");
    
}

