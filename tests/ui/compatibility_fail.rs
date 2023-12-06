use broadcastable::{Bound, Compatible};
use typenum::{U2, U3};

fn main() {
    // 2 and 3 are incompatible because they are not equal and neither is 1
    type FailedAlready = <(U2, U3) as Compatible>::Bound;
    println!("Bound: {}", FailedAlready::bound());
}
