// https://stackoverflow.com/a/56829792/22234700

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

fn fix(source: impl Fn(Rc<dyn Fn(u32) -> u32>, u32) -> u32 + 'static) -> Rc<dyn Fn(u32) -> u32> {
    let weak_holder: Rc<RefCell<Weak<dyn Fn(u32) -> u32>>> =
        Rc::new(RefCell::new(Weak::<fn(u32) -> u32>::new()));
    let weak_holder2 = weak_holder.clone();
    let f: Rc<dyn Fn(u32) -> u32> = Rc::new(move |x| {
        let f = weak_holder2.borrow().upgrade().unwrap();
        source(f, x)
    });
    weak_holder.replace(Rc::downgrade(&f));
    f
}

fn main() {
    let fact = fix(|fact, x| if x == 0 { 1 } else { x * fact(x - 1) });

    println!("{}", fact(5)); // prints "120"
    println!("{}", fact(6)); // prints "720"
}
