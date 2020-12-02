enum List {
    Cons(i32, Box<List>),
    Nil,
}

use std::ops::Deref;

struct MyBox<D>(D);

impl<D> MyBox<D> {
    fn new(x: D) -> MyBox<D> {
        MyBox(x)
    }
}

impl<D> Deref for MyBox<D> {
    type Target = D;

    fn deref(&self) -> &D {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}


use crate::List::{Cons, Nil};
use std::collections::HashMap;

fn main() {
    // let x = 5;
    // let y = MyBox::new(x);
    // let s = MyBox::new(String::from("Rust"));

    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    // hello(&s);

    // let c = CustomSmartPointer{
    //     data: String::from("my stuff"),
    // };

    // drop(c);

    // let d = CustomSmartPointer {
    //     data: String::from("other stuff"),
    // };

    // println!("CustomSmartPointers created!");

    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));

}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn get_unique(vetor: &[i32]) -> i32 {
    let mut m = HashMap::new();

    for element in vetor {
        m.insert(element, 1 + if m.contains_key(element) { m[element] } else { 0 });
    }

    for (key, val) in m {
        if val == 1 { println!("O número {} é único", key); }
    }
    
    0
}