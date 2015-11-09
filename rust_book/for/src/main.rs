fn main() {
    println!("Hello, world!");
    //src: https://doc.rust-lang.org/nightly/book/loops.html#enumerate
    for (i,j) in (5..10).enumerate() {
        println!("i = {} and j = {}", i, j);//i=0 , j=5
    }

    let lines = "hello\nworld".lines();
    for (linenumber, line) in lines.enumerate() {
        println!("{}: {}", linenumber, line);
    }

    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; } // continues the loop over x
            if y % 2 == 0 { continue 'inner; } // continues the loop over y
            println!("x: {}, y: {}", x, y);
        }
    }
}
