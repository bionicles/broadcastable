use typenum::{Cmp, Equal, Greater, IsEqual, Less, B1, U1, U2, U3};

// Define the Bound trait
pub trait Bound: typenum::Unsigned + std::fmt::Debug {
    fn bound() -> usize;
}

// Implement Bound for U1, U2, U3
impl Bound for U1 {
    fn bound() -> usize {
        1
    }
}
impl Bound for U2 {
    fn bound() -> usize {
        2
    }
}
impl Bound for U3 {
    fn bound() -> usize {
        3
    }
}

/// user-friendly bounds check on tuples
pub trait Compatible {
    /// Resulting bound if compatible
    type Bound: Bound;
}

impl<N, M> Compatible for (N, M)
where
    N: Bound + Cmp<M>,
    M: Bound,
    <N as Cmp<M>>::Output: CompatibleBounds<N, M>,
{
    // the output here is the bound to use in the new result shape
    type Bound = <<N as Cmp<M>>::Output as CompatibleBounds<N, M>>::Output;
}

/// Trait to check if two bounds can broadcast and to get the resulting shape after broadcasting.
pub trait CompatibleBounds<N, M>
where
    N: Bound,
    M: Bound,
{
    /// resulting bound if compatible
    type Output: Bound;
}
// N = M, output will be N
impl<N, M> CompatibleBounds<N, M> for Equal
where
    N: Bound + Cmp<M, Output = Self>,
    M: Bound,
{
    type Output = N;
}

// N < M and N is 1, output will be M
impl<N, M> CompatibleBounds<N, M> for Less
where
    N: Bound + IsEqual<U1, Output = B1>,
    M: Bound + Cmp<N, Output = Greater>,
{
    type Output = M;
}
// N > M and M is 1, output will be N
impl<N, M> CompatibleBounds<N, M> for Greater
where
    N: Bound + Cmp<M, Output = Self>,
    M: Bound + IsEqual<U1, Output = B1>,
{
    type Output = N;
}

#[cfg(test)]
mod tests_for_nd {

    #[test]
    fn test_compile_failures() {
        // Create a new instance of TestCases
        let t = trybuild::TestCases::new();
        // Compatible should not compile with incompatible bounds
        t.compile_fail("tests/ui/compatibility_fail.rs");
    }
}

// testing an alternative with `compile_error` to see if we can make more helpful messages

// // A trait to represent boolean values
// pub trait Boolean {}

// // Define two types representing possible boolean values
// pub struct Yes;
// pub struct No;

// impl Boolean for Yes {}
// impl Boolean for No {}

// // Conditional logic
// pub trait If<Condition: Boolean> {
//     type Output;
// }

// // Implementation when the condition is Yes
// impl<LeftYes, RightNo> If<Yes> for (LeftYes, RightNo) {
//     type Output = LeftYes;
// }

// // Implementation when the condition is No
// impl<LeftYes, RightNo> If<No> for (LeftYes, RightNo) {
//     type Output = RightNo;
// }

// // Example usage in CompatibleBounds implementations
// impl<N, M> CompatibleBounds<N, M> for Less
// where
//     N: Bound + IsEqual<U1, Output = B1>,
//     M: Bound,
// { // compile fails BEFORE it's possible to conditionally check if N < M, so this doesn't work
//     type Output = <(M, compile_error!("Incompatible bounds")) as If<N::Output>>::Output;
// }

// impl<N, M> CompatibleBounds<N, M> for Greater
// where
//     N: Bound,
//     M: Bound + IsEqual<U1, Output = B1>,
// {
//     type Output = <(N, compile_error!("Incompatible bounds")) as If<M::Output>>::Output;
// }

// thus, we can't really make helpful error messages in type level rust right now.
