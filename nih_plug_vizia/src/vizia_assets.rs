//! Registration functions for Vizia's built-in fonts. These are not enabled by default in
//! `nih_plug_vizia` to save on binary size.

// TODO: vizia: fonts mod is private now, not sure what to do
//       vizia only embeds the three roboto variants and icon set now so it might not be so bad on binary size anymore

use vizia::prelude::*;

// This module provides a re-export and simple font wrappers around the re-exported fonts.

// TODO: vizia: see above
// pub use vizia::fonts;

/// The font name for the Roboto (Regular) font, needs to be registered using [`register_roboto()`]
/// first.
pub const ROBOTO: &str = "Roboto";
/// The font name for the Roboto Bold font, needs to be registered using [`register_roboto_bold()`]
/// first.
pub const ROBOTO_BOLD: &str = "Roboto Bold";
/// The font name for the icon font (Entypo), needs to be registered using [`register_icons()`]
/// first.
pub const ICONS: &str = "Tabler Icons";

pub fn register_roboto(cx: &mut Context) {
    // cx.add_fonts_mem(&[fonts::ROBOTO_REGULAR]);
}
pub fn register_roboto_bold(cx: &mut Context) {
    // cx.add_fonts_mem(&[fonts::ROBOTO_BOLD]);
}
pub fn register_icons(cx: &mut Context) {
    // cx.add_fonts_mem(&[fonts::TABLER_ICONS]);
}