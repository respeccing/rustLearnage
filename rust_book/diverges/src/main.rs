fn main() {
    fn diverges() -> ! {
       panic!("This function never returns!");
    }
    let x: i32 = diverges();
    let x: String = diverges();
    
}

