use std::rc::Rc;
use std::cell::RefCell;
use std::rc::Weak;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    child: RefCell<Vec<Rc<Node>>>
}

fn main() {
    let leaf = Rc::new(Node {
        value: 10,
        parent: RefCell::new(Weak::new()),
        child: RefCell::new(vec![])
    });
    // convert from weak to rc ref as the weak reference is not aware of the ownership
    // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!("strong count = {}", Rc::strong_count(&leaf));
    println!("weak count = {}", Rc::weak_count(&leaf));

    {

    let parent = Rc::new(Node {
        value:15,
        parent: RefCell::new(Weak::new()),
        child: RefCell::new(vec![Rc::clone(&leaf)])
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&parent);
    // println!("{:?}", leaf);
    // println!("{:?}", parent);

    println!("strong count = {}", Rc::strong_count(&leaf));
    println!("weak count = {}", Rc::weak_count(&leaf));

    println!("strong count = {}", Rc::strong_count(&parent));
    println!("weak count = {}", Rc::weak_count(&parent));
}


println!("strong count = {}", Rc::strong_count(&leaf));
println!("weak count = {}", Rc::weak_count(&leaf));

    // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());


}
