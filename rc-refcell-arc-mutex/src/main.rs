use std::cell::Ref;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}


fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    //checking if there is any weak pointer
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("Strong pointer of leaf at 1: {}", Rc::strong_count(&leaf));
    println!("Weak pointer of leaf at 1: {}", Rc::weak_count(&leaf));

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("Strong pointer of leaf at 2: {}", Rc::strong_count(&leaf));
    println!("Weak pointer of leaf at 2: {}", Rc::weak_count(&leaf));
    println!("Strong pointer of branch at 3: {}", Rc::strong_count(&branch));
    println!("Weak pointer of branch at 3: {}", Rc::weak_count(&branch));
    println!("leaf parent = {:#?}", leaf.parent.borrow().upgrade());


}