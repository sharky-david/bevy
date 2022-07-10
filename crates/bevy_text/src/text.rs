use bevy_asset::Handle;
use bevy_ecs::{prelude::Component, reflect::ReflectComponent};
use bevy_reflect::{prelude::*, FromReflect};
use bevy_render::color::Color;
use serde::{Deserialize, Serialize};

use crate::Font;

#[derive(Component, Debug, Default, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct Text {
    pub sections: Vec<TextSection>,
    pub alignment: TextAlignment,
}

impl Text {
    /// Constructs a [`Text`] with (initially) one section.
    ///
    /// ```
    /// # use bevy_asset::{AssetServer, Handle};
    /// # use bevy_render::color::Color;
    /// # use bevy_text::{Font, Text, TextAlignment, TextStyle, HorizontalAlign, VerticalAlign};
    /// #
    /// # let font_handle: Handle<Font> = Default::default();
    /// #
    /// // basic usage
    /// let hello_world = Text::with_section(
    ///     "hello world!".to_string(),
    ///     TextStyle {
    ///         font: font_handle.clone(),
    ///         font_size: 60.0,
    ///         color: Color::WHITE,
    ///     },
    ///     TextAlignment {
    ///         vertical: VerticalAlign::Center,
    ///         horizontal: HorizontalAlign::Center,
    ///     },
    /// );
    ///
    /// let hello_bevy = Text::with_section(
    ///     // accepts a String or any type that converts into a String, such as &str
    ///     "hello bevy!",
    ///     TextStyle {
    ///         font: font_handle,
    ///         font_size: 60.0,
    ///         color: Color::WHITE,
    ///     },
    ///     // you can still use Default
    ///     Default::default(),
    /// );
    /// ```
    pub fn with_section<S: Into<String>>(
        value: S,
        style: TextStyle,
        alignment: TextAlignment,
    ) -> Self {
        Self {
            sections: vec![TextSection {
                value: value.into(),
                style,
            }],
            alignment,
        }
    }

    /// Constructs a [`Text`] with one or more sections and (initially) the same style for every
    /// section.
    ///
    /// ```
    /// # use bevy_asset::{AssetServer, Handle};
    /// # use bevy_render::color::Color;
    /// # use bevy_text::{Font, Text, TextAlignment, TextStyle, HorizontalAlign, VerticalAlign};
    /// #
    /// # let font_handle: Handle<Font> = Default::default();
    /// #
    /// // basic usage
    /// let hello_world = Text::with_sections(
    ///     vec!["hello ", "world!"],
    ///     TextStyle {
    ///         font: font_handle.clone(),
    ///         font_size: 60.0,
    ///         color: Color::WHITE,
    ///     },
    ///     TextAlignment {
    ///         vertical: VerticalAlign::Center,
    ///         horizontal: HorizontalAlign::Center,
    ///     },
    /// );
    ///
    /// // hello & world both have the same TextStyle
    /// let hello = &hello_world.sections[0];
    /// let world = &hello_world.sections[1];
    ///
    /// let hello_bevy = Text::with_sections(
    ///     // accepts Strings or any type that converts into a String, such as &str
    ///     vec!["hello ", "bevy!"],
    ///     TextStyle {
    ///         font: font_handle,
    ///         font_size: 60.0,
    ///         color: Color::WHITE,
    ///     },
    ///     // you can still use Default
    ///     Default::default(),
    /// );
    /// ```
    pub fn with_sections<S: Into<String>>(
        values: Vec<S>,
        style: TextStyle,
        alignment: TextAlignment
    ) -> Self {
        Self {
            sections: values.into_iter().map(|v| TextSection {
                value: v.into(),
                style: style.clone()
            }).collect(),
            alignment
        }
    }
}

#[derive(Debug, Default, Clone, FromReflect, Reflect)]
pub struct TextSection {
    pub value: String,
    pub style: TextStyle,
}

impl TextSection {
    pub fn new<S: Into<String>>(value: S, style: TextStyle) -> Self {
        Self {
            value: value.into(),
            style
        }
    }
}

#[derive(Debug, Clone, Copy, Reflect)]
pub struct TextAlignment {
    pub vertical: VerticalAlign,
    pub horizontal: HorizontalAlign,
}

impl Default for TextAlignment {
    fn default() -> Self {
        TextAlignment {
            vertical: VerticalAlign::Top,
            horizontal: HorizontalAlign::Left,
        }
    }
}

/// Describes horizontal alignment preference for positioning & bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
#[reflect_value(Serialize, Deserialize)]
pub enum HorizontalAlign {
    /// Leftmost character is immediately to the right of the render position.<br/>
    /// Bounds start from the render position and advance rightwards.
    Left,
    /// Leftmost & rightmost characters are equidistant to the render position.<br/>
    /// Bounds start from the render position and advance equally left & right.
    Center,
    /// Rightmost character is immetiately to the left of the render position.<br/>
    /// Bounds start from the render position and advance leftwards.
    Right,
}

impl From<HorizontalAlign> for glyph_brush_layout::HorizontalAlign {
    fn from(val: HorizontalAlign) -> Self {
        match val {
            HorizontalAlign::Left => glyph_brush_layout::HorizontalAlign::Left,
            HorizontalAlign::Center => glyph_brush_layout::HorizontalAlign::Center,
            HorizontalAlign::Right => glyph_brush_layout::HorizontalAlign::Right,
        }
    }
}

/// Describes vertical alignment preference for positioning & bounds. Currently a placeholder
/// for future functionality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
#[reflect_value(Serialize, Deserialize)]
pub enum VerticalAlign {
    /// Characters/bounds start underneath the render position and progress downwards.
    Top,
    /// Characters/bounds center at the render position and progress outward equally.
    Center,
    /// Characters/bounds start above the render position and progress upward.
    Bottom,
}

impl From<VerticalAlign> for glyph_brush_layout::VerticalAlign {
    fn from(val: VerticalAlign) -> Self {
        match val {
            VerticalAlign::Top => glyph_brush_layout::VerticalAlign::Top,
            VerticalAlign::Center => glyph_brush_layout::VerticalAlign::Center,
            VerticalAlign::Bottom => glyph_brush_layout::VerticalAlign::Bottom,
        }
    }
}

#[derive(Clone, Debug, Reflect, FromReflect)]
pub struct TextStyle {
    pub font: Handle<Font>,
    pub font_size: f32,
    pub color: Color,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            font: Default::default(),
            font_size: 12.0,
            color: Color::WHITE,
        }
    }
}

impl TextStyle {

    pub fn new(font: &Handle<Font>, font_size: f32, color: Color) -> Self {
        Self {
            font: (*font).clone(),
            font_size,
            color
        }
    }

    /// Creates a new ['TextStyle'], with the given font, and all other properties copied from this
    /// ['TextStyle']
    ///
    /// ```
    /// use bevy_text::{Font, TextStyle};
    ///
    /// let base_style = TextStyle::default();
    /// let fancy_font = asset_server::load("fancy_font.ttf");
    /// let fancy_font_style = base_style.clone_with_font(fancy_font);
    ///
    /// assert_ne!(
    ///     base_style,
    ///     fancy_font_style
    /// )
    /// ```
    pub fn clone_with_font(&self, font: Handle<Font>) -> Self {
        Self {
            font: (*font).clone(),
            font_size: self.font_size,
            color: self.color
        }
    }

    /// Creates a new ['TextStyle'], with the given font size, and all other properties copied from
    /// this ['TextStyle']
    ///
    /// ```
    /// use bevy_text::TextStyle;
    ///
    /// let base_style = TextStyle::default();
    /// let large_text_style = base_style.clone_with_font_size(40.0);
    ///
    /// assert_ne!(
    ///     base_style,
    ///     large_text_style
    /// )
    /// ```
    pub fn clone_with_font_size(&self, font_size: f32) -> Self {
        Self {
            font: self.font.clone(),
            font_size,
            color: self.color
        }
    }

    /// Creates a new ['TextStyle'], with the given font, and all other properties copied from this
    /// ['TextStyle']
    ///
    /// ```
    /// use bevy_text::TextStyle;
    /// use bevy_render::color::Color;
    ///
    /// let base_style = TextStyle::default();
    /// let pink_text_style = base_style.clone_with_color(Color::PINK);
    ///
    /// assert_ne!(
    ///     base_style,
    ///     pink_text_style
    /// )
    /// ```
    pub fn clone_with_color(&self, color: Color) -> Self {
        Self {
            font: self.font.clone(),
            font_size: self.font_size,
            color
        }
    }

}

#[cfg(test)]
mod test {
    use bevy_asset::HandleId;
    use bevy_reflect::TypeUuid;
    use super::*;

    fn text_with_sections_styles_match() {
        let text = Text::with_sections(
            vec!["hello ", "world"],
            TextStyle {
                font: Default::default(),
                font_size: 20.0,
                color: Color::ALICE_BLUE,
            },
            Default::default()
        );
        assert_eq!(
            text.sections[0].style,
            text.sections[1].style
        )
    }

    fn clone_style_with_font() {
        let base_style = TextStyle::default();
        let new_handle: Handle<Font> = Handle::weak(HandleId::random::<Font>());
        let new_style = base_style.clone_with_font(new_handle);
        assert_ne!(
            base_style,
            new_style
        )
    }

    fn clone_style_with_size() {
        let base_style = TextStyle::default();
        let new_style = base_style.clone_with_font_size(40.0);
        assert_ne!(
            base_style,
            new_style
        )
    }

    fn clone_style_with_color() {
        let base_style = TextStyle::default();
        let new_style = base_style.clone_with_color(Color::PINK);
        assert_ne!(
            base_style,
            new_style
        )
    }
}