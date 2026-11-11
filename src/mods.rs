//! Written by the recompiler: the hand-written files in `mods/`.
#![allow(warnings)]

#[path = "../mods/frame_limiter.rs"]
pub mod frame_limiter;
pub use frame_limiter::*;
#[path = "../mods/german_fallback.rs"]
pub mod german_fallback;
pub use german_fallback::*;
#[path = "../mods/give_up.rs"]
pub mod give_up;
pub use give_up::*;
#[path = "../mods/mission_select.rs"]
pub mod mission_select;
pub use mission_select::*;
