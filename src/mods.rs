//! Written by the recompiler: the hand-written files in `mods/`.
#![allow(warnings)]

#[path = "../mods/frame_limiter.rs"]
pub mod frame_limiter;
pub use frame_limiter::*;
#[path = "../mods/game_font.rs"]
pub mod game_font;
pub use game_font::*;
#[path = "../mods/german_fallback.rs"]
pub mod german_fallback;
pub use german_fallback::*;
#[path = "../mods/give_up.rs"]
pub mod give_up;
pub use give_up::*;
#[path = "../mods/language_check.rs"]
pub mod language_check;
pub use language_check::*;
#[path = "../mods/mission_select.rs"]
pub mod mission_select;
pub use mission_select::*;
