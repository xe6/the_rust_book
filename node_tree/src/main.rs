
#![allow(unused_variables)]
fn main() {
use std::rc::{Rc, Weak};
use std::cell::RefCell;

let leaf = Rc::new(Node {
    value: 3,
    children: RefCell::new(vec![]),
    parent: RefCell::new(Weak::new()),
});

println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

let branch = Rc::new(Node {
    value: 5,
    children: RefCell::new(vec![Rc::clone(&leaf)]),
    parent: RefCell::new(Weak::new()),
});

*leaf.parent.borrow_mut() = Rc::downgrade(&branch);

println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
}
