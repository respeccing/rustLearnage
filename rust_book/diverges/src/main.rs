fn main() {
    //src: https://doc.rust-lang.org/nightly/book/functions.html#diverging-functions
    fn diverges() -> ! {
       panic!("This function never returns!");
    }
    let x: i32 = diverges();
    let x: String = diverges();
    
}

