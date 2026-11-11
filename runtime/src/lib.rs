//! The runtime that recompiled games run on. This is original code: it holds
//! nothing from any game. Generated code lives in the game's `-rust` repo.

pub mod cpu;
pub mod host;
pub mod loader;
pub mod pe;
pub mod sys;

pub use cpu::{Cpu, GuestFn};
