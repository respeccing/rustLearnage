fn main() {
    println!("Hello, tuples!");
    let x = (1, "hello");
    let x: (i32, &str) = (1, "hello");

    let mut x = (1, 2); // x: (i32, i32)
    let y = (2, 3); // y: (i32, i32)

    x = y;

    let (x, y, z) = (1, 2, 3);
    println!("x is {}", x);

    let (x,) = (1,); //single-element tuple   destructuring
    println!("x is {}", x);


    //indexing, src: https://doc.rust-lang.org/nightly/book/primitive-types.html#tuple-indexing
    let tuple = (1, 2, 3);

    let x = tuple.0;
    let y = tuple.1;
    let z = tuple.2;

    println!("z is {}", z);

    //further info: https://doc.rust-lang.org/nightly/std/primitive.tuple.html

    let x=1;
    let y=(5,6);
//    println!("{}", y.x);//FIXME: can probably be done with macros, when I learn them!
}
