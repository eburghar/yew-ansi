//! ANSI escape code rendering for the web
//!
//! # Yew
//!
//! ```
//! # use yew::html;
//! # use yew_ansi::Ansi;
//! html! {
//!     <Ansi text="Hello \u{001b}[32mWorld\u{001b}[39;1m!" />
//! }
//! # ;
//! ```
//!
//! This will generate the following output (whitespace added for clarity):
//!
//! ```html
//! <pre style="font-family: monospace">
//!     <span>Hello </span>
//!     <span style="color:#00ff00;">World</span>
//!     <span style="font-weight:bold;">!</span>
//! </pre>
//! ```
//!
//! Refer to [`Ansi`] and [`AnsiProps`] for more information about the component.

pub use cursor::CharCursor;
pub use graphic_rendition::*;
pub use sequences::*;
pub use style::*;

#[cfg(feature = "yew")]
pub use yew_component::*;

mod cursor;
mod graphic_rendition;
mod sequences;
mod style;
#[cfg(feature = "yew")]
mod yew_component;

/// Iterator over the SGR segments in a string slice.
///
/// Returned by [`get_sgr_segments`].
pub struct SgrSegmentIter<'a> {
    markers: MarkerIter<'a>,
    effect: SgrEffect,
}
impl<'a> SgrSegmentIter<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            markers: get_markers(s),
            effect: SgrEffect::default(),
        }
    }
}
impl<'a> Iterator for SgrSegmentIter<'a> {
    type Item = (SgrEffect, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.markers.next()? {
                Marker::Text(text) => {
                    return Some((self.effect.clone(), text));
                }
                Marker::Sequence(Escape::Csi(Csi::Sgr(sgrs))) => {
                    self.effect.apply_sgrs(sgrs);
                }
            }
        }
    }
}

/// Create an iterator which iterates over SGR segments in a string slice.
/// Each item consists of a [`SgrEffect`] and the corresponding text slice it applies to.
///
/// ```
/// # use yew_ansi::*;
/// let mut segments = yew_ansi::get_sgr_segments("Hello \u{001b}[32mWorld\u{001b}[39;1m!");
/// assert_eq!(segments.next(), Some((SgrEffect::default(), "Hello ")));
/// assert_eq!(
///     segments.next(),
///     Some((
///         SgrEffect {
///             fg: ColorEffect::Name(ColorName::Green),
///             ..Default::default()
///         },
///         "World"
///     ))
/// );
/// assert_eq!(
///     segments.next(),
///     Some((
///         SgrEffect {
///             bold: true,
///             ..Default::default()
///         },
///         "!"
///     ))
/// );
/// ```
pub fn get_sgr_segments(s: &str) -> SgrSegmentIter {
    SgrSegmentIter::new(s)
}
