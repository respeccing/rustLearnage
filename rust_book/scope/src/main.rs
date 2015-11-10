//src: https://doc.rust-lang.org/nightly/book/lifetimes.html#thinking-in-scopes
struct Foo<'a> {
    x: &'a i32,
}

struct Foo2 {
    x: i32,
}

fn main() {
    let x;                    // -+ x goes into scope
                              //  |
    {                         //  |
        let y = &5;           // ---+ y goes into scope
        let f = Foo2 { x: y }; // ---+ f goes into scope
        x = &f.x;             //  | | error here
    }                         // ---+ f and y go out of scope
                              //  |
    println!("{}", x);        //  |
}
