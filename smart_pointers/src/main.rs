use smart_pointers::CustomSmartPointer;
use smart_pointers::List;
use smart_pointers::List::{Cons, Nil};
use smart_pointers::MyBox;
use std::mem::drop;
use std::rc::Rc;

fn main() {
    smart_pointer_dereference();
    smart_ponter_drop();
    reference_counted_pointer();
}

fn smart_pointer_dereference() {
    let m = MyBox::new(String::from("Rust"));
    // dereferences MyBox -> String -> &str automatically
    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn smart_ponter_drop() {
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    let e = CustomSmartPointer {
        data: String::from("some data"),
    };

    println!("CustomSmartPointers created.");
    drop(e);
    println!("CustomerSmartPointer dropped before end of main");

    // c and d get dropped here
}

fn reference_counted_pointer() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope {}", Rc:: strong_count(&a));
}
