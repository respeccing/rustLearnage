//src: http://stackoverflow.com/a/29168659/539686
#![feature(core_intrinsics)]

fn print_type_of<T>(_: &T) -> () {
    let type_name =
        unsafe {
            std::intrinsics::type_name::<T>()
        };
    println!("{}", type_name);
}

fn main() -> () {
    print_type_of(&32.90);           // prints "f64"
    print_type_of(&(vec!(1, 2, 4))); // prints "collections::vec::Vec<i32>"
    let x = 5; // x: i32 //src: https://doc.rust-lang.org/nightly/book/variable-bindings.html
    print_type_of(&x);
}

