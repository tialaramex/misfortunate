# Misfortunate

Misfortunate is a collection of perverse implementations of safe Rust traits

## Perverse?

Rust's safe traits come with some reasonable expectation for how they're intended to work. But unlike *unsafe* traits the language promises your program does not have Undefined Behaviour even if you don't obey.

Misfortunate provides implementations which conform to any function signatures for the trait, and so can indeed be used as implementations of that trait, yet they defy the expectations set out in the standard.

Example: [Eq](https://doc.rust-lang.org/std/cmp/trait.Eq.html) is a trait promising that this type exhibits equivalence, and [Hash](https://doc.rust-lang.org/std/hash/trait.Hash.html) is a trait promising a
type can be hashed in a consistent way. Misfortunate's `Maxwell` type implements both and yet every instance has an identical hash, while none of them are equal to each other or themselves.

## C++

If you have extensive experience with C++ this might seem like a weird idea. Unlike Rust's traits, each C++ concept is defined only by its syntax. Any semantics are a matter for the programmer to consider.
So for example in C++ float is `std::totally_ordered` and it is the responsibility of a programmer using an algorithm that requires `std::totally_ordered` types with floats to ensure they are never NaN. Whereas
in Rust `f32` is not `Ord` and so you won't mistakenly try to sort a `Vec` of NaNs.

As a result a collection of types like this one wouldn't make much sense in C++. In Rust however, types which do not in fact exhibit the desired characteristics will not generally implement a trait at all and
the types presented here do so for your amusement and enlightenment.


## Traits so far

`std::cmp::Eq` is implemented in Always, Echo, Everything, Funhouse, Jumble, Maxwell, Mirror, Nothing, Oneway{Equal, Greater, Less}, Reverse

`std::cmp::Ord` is implemented in Always, Echo, Jumble, Oneway{Equal, Greater, Less}, Reverse

`std::hash::Hash` is implemented in Maxwell

`std::io::{Read, Write}` are implemented in BlackHole, LoadLetter

`std::fmt::Write` is implemented in BlackHole, Pristine

`std::borrow::{Borrow, BorrowMut}` are implemented in Loaner

`std::iter::ExactSizeIterator` is implemented in Comte

`std::clone::Clone` is implemented in Multiplicity

`std::iter::Extend` is implemented in BlackHole

`std::iter::FromIterator` is implemented in BlackHole

`std::iter::Sum` is implemented in Nice, Nothing

`std::iter::Product` is implemented in Nice

`std::ops::{Deref, DerefMut}` are implemented in Double

`std::str::FromStr` is implemented in BlackHole

`std::ops::RangeBounds` is implemented in Everything, Nothing

`std::ops::Index` is implemented in Nothing

NB Some of the traits may necessarily also be implemented but in a more conventional way for structures they don't list


## Nightly

Some traits that would be potentially interesting aren't stable Rust features at time of writing and so I did not write implementations for Misfortunate. These include

* Pattern
* SlicePattern
* AsyncIterator
* Step

## Ideas

A type with random Order behaviour, perhaps named Tombola ?

