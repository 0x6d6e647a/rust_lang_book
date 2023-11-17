use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List<T> {
    Cons(T, RefCell<Rc<List<T>>>),
    Nil,
}

impl<T> List<T> {
    fn tail(&self) -> Option<&RefCell<Rc<List<T>>>> {
        match self {
            Cons(_, item) => Some(item) ,
            Nil => None,
        }
    }
}

use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b create = {}", Rc::strong_count(&a));
    println!("b inital rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("a rc count after changing = {}", Rc::strong_count(&a));
    println!("b rc count after changing = {}", Rc::strong_count(&b));

    // Cycle causes stack overflow.
    // println!("a next item = {:?}", a.tail());
}
