pub trait Jump {
    type Previous;
    type Next;
}

/// # lysbilder
///
/// ## the slide system no one needed.
pub struct Slide1;
impl Jump for Slide1 {
    type Previous = ();
    type Next = Slide2;
}

/// # Who am I?
pub struct Slide2;
impl Jump for Slide2 {
    type Previous = Slide1;
    type Next = ();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
