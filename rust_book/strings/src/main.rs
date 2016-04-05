
//src: http://doc.rust-lang.org/book/strings.html

fn main() {
    let mut s = "Hello".to_string(); // mut s: String
    println!("{}", s);
    s.push_str(", world.");
    println!("{}", s);

    let s = "Hello".to_string();
//    takes_slice(s.as_slice());//FIXME: as_slice is now what?
    compare(s);

    let s1 = "Hi.�💖";
    let s2 = "Ok.💖💖";
    println!("{} {}", s1.len(), s2.len());//10 11
    println!("{} {}", s1.chars().count(), s2.chars().count());//5 5
}

fn takes_slice(slice: &str) {
    println!("Got: {}", slice);
}

fn compare(string: String) {//good way
    //Viewing a String as a &str is cheap, but converting the &str to a String involves allocating
    //memory.
    {//    if string.as_slice() == "Hello" {//FIXME: as_slice ?
        println!("yes");
    }
}

fn comparebad(string: String) {//"bad" way
    if string == "Hello".to_string() {
        println!("yes");
    }
}
