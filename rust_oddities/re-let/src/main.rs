
#![forbid(non_shorthand_field_patterns)]

#![warn(dead_code)]

#![warn(trivial_casts)]

#![allow(missing_docs)]

#![warn(unsafe_code)]


#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]

#![warn(box_pointers)]

#![warn(trivial_numeric_casts)]
#![warn(unstable_features)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]


fn main() {
    let x=10;
    println!("{}",x);
    let x=11; //TODO: want a (lint check) warning here
    println!("{}",x);
    {
        let x=12; //TODO: want a (lint check) warning here
        println!("{}",x);
    }
    println!("{}",x);
}
