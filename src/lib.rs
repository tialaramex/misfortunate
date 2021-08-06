#![warn(rust_2018_idioms)]

mod always;
pub use crate::always::Always;
mod blackhole;
pub use crate::blackhole::BlackHole;
mod funhouse;
pub use crate::funhouse::Funhouse;
mod loadletter;
pub use crate::loadletter::LoadLetter;
mod maxwell;
pub use crate::maxwell::Maxwell;
mod mirror;
pub use crate::mirror::Mirror;
mod oneway;
pub use crate::oneway::{OnewayGreater, OnewayLess, OnewayEqual};
mod pristine;
pub use crate::pristine::Pristine;
mod reverse;
pub use crate::reverse::Reverse;
mod loaner;
pub use crate::loaner::Loaner;
