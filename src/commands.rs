use crate::fsnodetypes::ListView;


pub type ManipFun = 
fn (files: Vec<usize>, list_view: &mut ListView ) -> Vec<usize>;
pub type TargetFun =
fn (list_view: &ListView, frompos: usize) -> Option<usize>;
pub type TargetlessFun =
fn (list_view: &mut ListView, pos: usize);

// Manip
pub fn yank(files: Vec<usize>, list_view: &mut ListView ) -> Vec<usize> {
    println!("YANK!!! {:?}", files);
    Vec::new()

}

// Target
pub fn up(list_view: &ListView, frompos: usize) -> Option<usize> {
    if frompos > 0 {Some(frompos-1)}
    else {None}
}


pub fn down(list_view: &ListView, frompos: usize) -> Option<usize> {
    if frompos < list_view.len() - 1 {Some(frompos+1)}
    else {None}
    
}

// Targetless
pub fn past(list_view: &mut ListView, pos: usize ) {
    println!("PAST!");
    
}

