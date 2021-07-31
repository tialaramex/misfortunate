# Misfortunate

Misfortunate is a collection of perverse implementations of safe Rust traits

## Perverse?

Rust's safe traits come with some reasonable expectation for how they're intended to work. But unlike *unsafe* traits the language promises your program does not have Undefined Behaviour even if you don't obey.

Misfortunate provides implementations which conform to any function signatures for the trait, and so can indeed be used as implementations of that trait, yet they defy the expectations set out in the standard.

Example: std::cmp::Eq is a trait promising that this type exhibits equivalence, and std::hash::Hash is a trait promising a type can be hashed in a consistent way. Misfortunate's [Maxwell] type implements both and yet
every instance has an identical hash, while none of them are equal to each other or themselves.

## Traits so far

Eq is implemented in Always, Funhouse, Maxwell, Mirror, Oneway{Equal, Greater, Less}, Reverse

Ord is implemented in Always, Oneway{Equal, Greater, Less}, Reverse

std::hash::Hash is implemented in Maxwell

std::io::{Read, Write} are implemented in BlackHole, LoadLetter

std::fmt::Write is implemented in BlackHole, Pristine
