///Always use these, in any rust source file (see 'rustc _Whelp' for all):

#![forbid(non_shorthand_field_patterns)]

#![warn(dead_code)]
///(maybe change to forbid, in real rust source files!)

#![forbid(unsafe_code)]

#![warn(missing_docs)]
///don't yet know how to add crate docs, so this will warn at the top of the file! (change to allow, for now)
///crate docs with: //! 

#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]

#![warn(box_pointers)]

#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unstable_features)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]


///Use clippy plugin: https://github.com/Manishearth/rust-clippy
///Add to Cargo.toml
/*
[dependencies]
clippy = "*"
*/
///Add to your .rs file:
#![feature(plugin)]
#![plugin(clippy)]

#![deny(clippy)]
#![deny(clippy_pedantic)]
//#![deny(shadow_unrelated)] //this shouldn't be needed if you also specify the clippy_pedantic group!


