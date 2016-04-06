fn main() {
    println!("Hello, world!");
    let a = "foobar";
    let b = "foo\
         bar";

    assert_eq!(a,b);
    //src: https://doc.rust-lang.org/reference.html#string-literals
}
