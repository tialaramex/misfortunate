# Misfortunate

Misfortunate is a collection of perverse implementations of safe Rust traits

## Perverse?

Rust's safe traits come with some reasonable expectation for how they're intended to work. But unlike *unsafe* traits the language promises your program does not have Undefined Behaviour even if you don't obey.

Misfortunate provides implementations which conform to any function signatures for the trait, and so can indeed be used as implementations of that trait, yet they defy the expectations set out in the standard.

Example: std::cmp::Eq is a trait promising that this type exhibits equivalence, and std::hash::Hash is a trait promising a type can be hashed in a consistent way. Misfortunate's [Maxwell] type implements both and yet
every instance has an identical hash, while none of them are equal to each other or themselves.

## C++

If you have extensive experience with C++ this might seem like a weird idea. Unlike Rust's traits, each C++ concept is defined only by its syntax. Any semantics are a matter for the programmer to consider.
So for example in C++ float is std::totally\_ordered and it is the responsibility of a programmer using an algorithm that requires std::totally\_ordered types with floats to ensure they are never NaN. Whereas
in Rust f32 is not Ord and so you won't mistakenly try to sort a Vec of NaNs.

As a result a collection of types like this one wouldn't make much sense in C++. In Rust however, types which do not in fact exhibit the desired characteristics will not generally implement a trait at all and
the types presented here do so for your amusement and enlightenment.



## Traits so far

Eq is implemented in Always, Funhouse, Maxwell, Mirror, Oneway{Equal, Greater, Less}, Reverse

Ord is implemented in Always, Oneway{Equal, Greater, Less}, Reverse

std::hash::Hash is implemented in Maxwell

std::io::{Read, Write} are implemented in BlackHole, LoadLetter

std::fmt::Write is implemented in BlackHole, Pristine

std::borrow::{Borrow, BorrowMut} are implemented in Loaner

ExactSizeIterator is implemented in Comte

Clone is implemented in Multiplicity

Extend is implemented in BlackHole

std::iter::FromIterator is implemented in BlackHole

std::ops::{Deref, DerefMut} are implemented in Double

