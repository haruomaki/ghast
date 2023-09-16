// https://stackoverflow.com/a/56829792/22234700

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

fn main() {
    let weak_holder: Rc<RefCell<Weak<dyn Fn(u32) -> u32>>> =
        Rc::new(RefCell::new(Weak::<fn(u32) -> u32>::new()));
    let weak_holder2 = weak_holder.clone();
    let fact: Rc<dyn Fn(u32) -> u32> = Rc::new(move |x| {
        let fact = weak_holder2.borrow().upgrade().unwrap();
        if x == 0 {
            1
        } else {
            x * fact(x - 1)
        }
    });
    weak_holder.replace(Rc::downgrade(&fact));

    println!("{}", fact(5)); // prints "120"
    println!("{}", fact(6)); // prints "720"
}
