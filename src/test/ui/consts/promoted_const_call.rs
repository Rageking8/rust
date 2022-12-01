#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
struct Panic;
impl const Drop for Panic { fn drop(&mut self) { panic!(); } }
pub const fn id<T>(x: T) -> T { x }
pub const C: () = {
    let _: &'static _ = &id(&Panic);
    //~^ ERROR: temporary value dropped while borrowed
    //~| ERROR: temporary value dropped while borrowed
};

fn main() {
    let _: &'static _ = &id(&Panic);
    //~^ ERROR: temporary value dropped while borrowed
    //~| ERROR: temporary value dropped while borrowed
}
