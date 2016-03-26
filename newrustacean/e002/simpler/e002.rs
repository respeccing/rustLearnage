/// from: https://raw.githubusercontent.com/chriskrycho/newrustacean.com/master/src/e002.rs


/// This struct is simple but useful to see how borrowing and moving work.
#[derive(Debug)]
pub struct Circle {
    /// X position of the circle's origin.
    pub x: f64,
    /// Y position of the circle's origin
    pub y: f64,
    /// Radius of the circle
    pub r: f64,

    pub w: Boxx,
}

#[derive(Debug)]
pub struct Boxx {
    pub x: f64,
    pub y: f64,
}

//const nil:Circle= Circle{x:1,y:2,r:3,w:nul};
//const nul:Circle= Circle{x:1,y:2,r:3,w:nil};


impl Drop for Circle {
    fn drop(&mut self) {
        println!("Dropping! {:?}",self);
    }
}

impl Circle {
    /// Creates a `Circle` instance centered on the "origin" (x = 0, y = 0).
    fn origin() -> Circle {
        Circle {
            x: 1.2,
            y: 3.4,
            r: 5.6,
            w: Boxx{x:1.0, y:2.0},
        }
    }

    pub fn by_borrow_x(&self) -> &f64 {
        println!("Taking ownership, not just borrowing a reference. INTENSE.");
        &self.x
    }

/*    pub fn by_take(self) -> Boxx {
        println!("Taking ownership, not just borrowing a reference. INTENSE.");
        self.w  //it is moved out... unless defined the trait Drop on the type Circle!
    }*/

    pub fn by_take2(self) -> f64 {
        println!("Taking ownership, not just borrowing a reference. INTENSE.");
        self.x //it is "moved?" out... and works even if Drop is defined on type Circle! (unlike for self.w)
            //ok, it is copied I guess, since self.x such a simple type
    }

    pub fn by_borrow(&self) -> &Boxx {
        println!("borrowing box");
        &self.w
    }
}

fn main() {
    // Constructors can have default behavior, like this one.
    let immutable = Circle::origin();

    {
        let b = immutable.by_borrow();
        println!("showing borrowed box: {:?}", b);
    }
    {
        let x = immutable.by_borrow_x();
        println!("showing borrowed x: {:?}", x);
    }
//    let y= immutable.by_take2();
    let by_take_box = immutable.by_take2();//TODO: what happened with y,z? or even x? they got destroyed? so the return is a copy/clone then? since self.w is returned! ok, so i'm assuming since the box is moved out, the others are just destroyed; but how is the box moved out? memmove? - ok, so, the value returned(self.x) seems to be copied! because when it's a Boxx then this won't be allowed with Drop impl. too!
    println!("By take: {:?}", by_take_box);
    //println!("Does the circle still exist? {:?}", immutable);//no
}

