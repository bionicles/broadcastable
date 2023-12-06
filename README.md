# Objective: Improve Type Level Rust in the Rust 2024 Edition

## TLDR: Demonstrate unhelpful compile failure errors with TypeNum

please note: **this is a small example**

_in my real use case, I also have HList (Cons/Nil Russian Doll) of TypeNums, so it's **much worse**!_

## Start Quick

```sh
gh repo clone bionicles/broadcastable && cd broadcastable && cargo test
```

```sh
cargo test
```

## Example of the Issue

Notice how **these TypeNums are really hard to read**:

```sh
┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈
error[E0271]: type mismatch resolving `<UInt<UInt<UTerm, B1>, B0> as IsEqual<UInt<UTerm, B1>>>::Output == B1`
 --> tests/ui/compatibility_fail.rs:7:27
  |
7 |     println!("Bound: {}", FailedAlready::bound());
  |                           ^^^^^^^^^^^^^ expected `B0`, found `B1`
  |
  = note: required for `typenum::Less` to implement `CompatibleBounds<UInt<UInt<UTerm, B1>, B0>, UInt<UInt<UTerm, B1>, B1>>`
┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈
```

**`rustc` displays full types even for complex nested types which represent abstractions**
(i.e. **instead of `U2` you see `UInt<UInt<UTerm, B1>, B0>`**)


## Workarounds Tried

1. I tried using **`tnfilt` from `https://github.com/auxoncorp/tnfilt`** -- but it **clobbers a lot of terminal style content like colors**, which **causes other issues with Rust compiler error readability in type-level programming**

2. I tried using **`compile_error!` (`https://doc.rust-lang.org/std/macro.compile_error.html`) in the `type Output = ...`** but unfortunately it **always fails to compile** which means we **can't conditionally trigger `compile_error!` inside trait impls**.

3. **`dfdx` (`https://github.com/coreylowman/dfdx`) does implement compile-time shape checking with const generics**, however this approach **only uses numerical values for the axis bounds**, which doesn't align with the principles of **Named Arrays (i.e. broadcast by axis names, not axis positions)**

## Motivations: Benefits of Type Level Linear Algebra

If this issue were fixed, then **Rust could have clearly better alternatives to numpy/jax/pytorch/tensorflow** etc,
because we could **detect "shape bugs" at compile time** instead of **running code and waiting to see if it crashes**.

Also, since we could **check broadcast alignment compatibility at compile time**, the **performance could dramatically improve**, since the **work could be done once and amortized over all the broadcasts during the runtime execution of the program** 

## License: MIT or Apache 2.0, at your option.

See LICENSE_MIT and/or LICENSE_APACHE for details.