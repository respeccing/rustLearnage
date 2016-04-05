//! The `adder` crate provides functions that add numbers to other numbers.
//!
//! # Examples
//!
//! ```
//! assert_eq!(4, adder::add_two(2));
//! ```
//!
//! (note: this only works in library crates, not binary crates)
//!
//! ```
//! #[test]
//! fn it_works3() {
//!     assert_eq!(4, add_two(2));
//! }
//! ```


/*#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
	   html_favicon_url = "https://doc.rust-lang.org/favicon.ico",
	   html_root_url = "https://doc.rust-lang.org/nightly/",
	   html_playground_url = "https://play.rust-lang.org/",
	   issue_tracker_base_url = "https://github.com/rust-lang/rust/issues/",
	   test(no_crate_inject, attr(allow(unused_variables), deny(warnings))))]
       */
#![doc(html_playground_url = "https://play.rust-lang.org/")]
//NOTE: ^ rustdoc --markdown-playground-url  WILL NOT WORK! only the above will! see: https://github.com/rust-lang/rust/pull/32746

#![feature(test)]
extern crate test;

//https://doc.rust-lang.org/nightly/book/testing.html#the-tests-module

#[test]
#[should_panic]
fn it_works0() {
    assert!(false);
    assert_eq!("Hello", "world");
}

#[test]
////#[should_panic(expected = "assertion failed")] // this works the same currently
#[should_panic(expected = "failed: false")]
fn it_works2() {
    assert!(false);
    assert_eq!("Hello", "world");
}


/*pub fn add_two(a: i32) -> i32 {
  a + 2
  }*/

#[test]
fn it_works3() {
    assert_eq!(4, add_two(2));
}


#[cfg(test)]
mod tests {
    //use super::add_two;
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }
}


/// This function adds two to its argument.
///
/// # Examples
///
/// ```
/// use adder::add_two;
///
/// assert_eq!(4, add_two(2));
/// ```
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests2 {
    use super::*; //glob feature
    use test::Bencher;
    use test;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| add_two(2));
        b.iter(|| {
            let mut n = 1000_u32;
            test::black_box(&mut n); // pretend to modify `n`
            (0..n).fold(0, |a, b| a ^ b)
        })
    }
}

