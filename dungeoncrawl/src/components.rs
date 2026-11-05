pub use crate::prelude::*;

// Render component describes how a player appears on the screen
#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

// Player component used as a tag to identify a player entity
#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Player;
