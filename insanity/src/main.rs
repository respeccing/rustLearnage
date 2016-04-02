fn main() {
    println!("Hello, insanity world!");
    let mut x=0;
    loop {
        println!("...doing the same thing over and over, expecting different results!");
        x+=1;
        if x >= 5 { break }
    }
    println!("Got them!");
}
