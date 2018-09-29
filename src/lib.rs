
pub trait Jump {
    type Previous;
    type Next;
}



/// # lysbilder
/// 
/// ## the slide system no one needed.
/// 
pub struct Slide1;
impl Jump for Slide1 {
    type Previous = ();
    type Next = Slide2;
}


/// # Who am I?
/// 
pub struct Slide2;
impl Jump for Slide2 {
    type Previous = Slide1;
    type Next = Slide3;
}


/// # This slide deck can hold so much code.
/// 
/// ```rust
/// extern crate panic_at_the_disco;
/// 
/// fn main() {
///     panic_at_the_disco::init();
/// 
///     panic!("at the disco");
/// }
/// ```
/// 
pub struct Slide3;
impl Jump for Slide3 {
    type Previous = Slide2;
    type Next = Slide4;
}


/// # The End
/// 
/// Please don't use it.
pub struct Slide4;
impl Jump for Slide4 {
    type Previous = Slide3;
    type Next = ();
}

