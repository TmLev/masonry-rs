// This software is licensed under Apache License 2.0 and distributed on an
// "as-is" basis without warranties of any kind. See the LICENSE file for
// details.

//! Font attributes

use crate::piet::{FontFamily, FontStyle, FontWeight};
use crate::Data;

/// A collection of attributes that describe a font.
///
/// This is provided as a convenience; library consumers may wish to have
/// a single type that represents a specific font face at a specific size.
#[derive(Debug, Clone, PartialEq)]
pub struct FontDescriptor {
    /// The font's [`FontFamily`](struct.FontFamily.html).
    pub family: FontFamily,
    /// The font's size.
    pub size: f64,
    /// The font's [`FontWeight`](struct.FontWeight.html).
    pub weight: FontWeight,
    /// The font's [`FontStyle`](struct.FontStyle.html).
    pub style: FontStyle,
}

impl FontDescriptor {
    /// Create a new descriptor with the provided [`FontFamily`].
    ///
    /// [`FontFamily`]: struct.FontFamily.html
    pub const fn new(family: FontFamily) -> Self {
        FontDescriptor {
            family,
            size: crate::piet::util::DEFAULT_FONT_SIZE,
            weight: FontWeight::REGULAR,
            style: FontStyle::Regular,
        }
    }

    /// Buider-style method to set the descriptor's font size.
    pub const fn with_size(mut self, size: f64) -> Self {
        self.size = size;
        self
    }

    /// Buider-style method to set the descriptor's [`FontWeight`].
    ///
    /// [`FontWeight`]: struct.FontWeight.html
    pub const fn with_weight(mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self
    }

    /// Buider-style method to set the descriptor's [`FontStyle`].
    ///
    /// [`FontStyle`]: enum.FontStyle.html
    pub const fn with_style(mut self, style: FontStyle) -> Self {
        self.style = style;
        self
    }
}

impl Default for FontDescriptor {
    fn default() -> Self {
        FontDescriptor {
            family: Default::default(),
            weight: Default::default(),
            style: Default::default(),
            size: crate::piet::util::DEFAULT_FONT_SIZE,
        }
    }
}

impl Data for FontDescriptor {
    fn same(&self, other: &Self) -> bool {
        self.family == other.family
            && self.size == other.size
            && self.weight == other.weight
            && self.style == other.style
    }
}
