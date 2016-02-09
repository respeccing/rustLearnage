use std::fmt;//for impl below

fn main() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);//31

    // Without a suffix, 31 becomes an i32. You can change what type 31 is,
    // with a suffix.
    println!("{:.*} days", 2, 31f32); //31.00
    println!("{:.*} days", 2, 31i32); //31 (no ".00" since it's not a float)
//    println!("{:.*} days with {0} zeroes", 2, 31i32); //invalid, trying to ref '2'

    let formatted_number = format!("{:.*}", 2, 1.234567);
    assert_eq!("1.23", formatted_number);

//    println!("{name} {} {}", 1, name = 2, 3);//can't put '3' after named arguments!

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    //the next arg '{}' has it's own iterator and so it's not affected by positional args '{x}'
    println!("{1} {} {2} {} {0} {} ", 0, 1, 2); // => "1 0 2 1 0 2"

    // As can named arguments.
    println!("{subject} {verb} {predicate}",
             predicate="over the lazy dog",
             subject="the quick brown fox",
             verb="jumps");

    // Special formatting can be specified after a `:`.
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number=1, width=6);
    println!("{:>width$}", 2, width=6);
    println!("{:>6}", 3);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number=1, width=6);

    // It will even check to make sure the correct number of arguments are
    // used.
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // done ^ Add the missing argument: "James"

    // Create a structure which contains an `i32`. Name it `Structure`.
    #[derive(Debug)] //makes this struct printable by {:?} (but still not by {}) see: https://rustbyexample.com/hello/print/print_debug.html
    struct Structure(i32);//done: how do I access this from fmt::Display below via self? self.0
/*    struct Structure{
        teh:i32
    };*/

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    // the trait `core::fmt::Display` is not implemented for the type `main::Structure
    println!("This struct `{}` won't print...ok now it will because fmt::Display trait is implemented for it!", Structure(3));//done: how do I make this work anyway?
    // done ^ Comment out this line.


    //doesn't matter that that impl is done later on
	impl fmt::Display for Structure {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			// The `f` value implements the `Write` trait, which is what the
			// write! macro is expecting. Note that this formatting ignores the
			// various flags provided to format strings.
			write!(f, "Structure({})moo", self.0) //self.0 is in 1.2.2 https://rustbyexample.com/hello/print/print_display.html (lol, found by google! since I wasn't that far in)
		}
	}
//impl core::fmt::Debug for Structure {//FIXME: how do I do this? without the #[derive(Debug)] above
//
//}
    let a=Structure(2);
    println!("These structs `{:?}` `{:?}` will print.", a, Structure(3));

    println!("Pi={:.*}", 2, 22f32/7f32);
    let pi:f32 = 22.0 / 7.0;//can't just do: 22/7; must use .0 or f32 or combinations;
    println!("Pi={:.*}", 2, pi);
//    println!("lowerhex={0:x} {0:X}", 43);//won't work
    //see: https://doc.rust-lang.org/std/fmt/
    println!("lowerhex={:x}", 43);
    println!("upperhex={:X}", 43);
    println!("binary={:b}", 43);
    println!("lowerhex with leading spaces:{:6x}", 43);
    println!("lowerhex with leading spaces:{:width$x}", 43, width=6);
    println!("debug with leading spaces:{:width$?}", 43, width=6);
    //FIXME: leading spaces for {:?} has no effect!
    println!("debug with leading spaces:{:width$?} has no effect", a, width=15);
    println!("debug with leading spaces:{:25?} has no effect", a);
}

