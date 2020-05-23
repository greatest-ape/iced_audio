//! Various styles for the [`HSlider`] widget
//! 
//! [`HSlider`]: ../native/h_slider/struct.HSlider.html

use iced::Color;
use iced_native::image;

use crate::TexturePadding;

/// The appearance of an [`HSlider`].
///
/// * `Texture` - uses an image texture for the handle
/// * `Classic` - modeled after hardware sliders
/// * `Rect` - a modern style with a line inside a filled rectangle
/// * `RectBipolar` - same as `Rect` but can have different colors for left,
/// right, and center positions
///
/// [`HSlider`]: ../../native/h_slider/struct.HSlider.html
#[derive(Debug, Clone)]
pub enum Style {
    /// uses an image texture for the handle
    Texture(TextureStyle),
    /// modeled after hardware sliders
    Classic(ClassicStyle),
    /// a modern style with a line inside a filled rectangle
    Rect(RectStyle),
    /// same as `Rect` but can have different colors for left,
    /// right, and center positions
    RectBipolar(RectBipolarStyle),
}

/// A [`Style`] for an [`HSlider`] that uses an image texture for the handle
///
/// * `rail_colors` - colors of the top and bottom of the rail
/// * `texture` - the [`Handle`] to the image texture
/// * `handle_width` - the width of the handle, not including padding
/// * `texture_padding` - the texture padding around the handle bounding
/// rectangle. This is useful when the texture is of a glowing handle or has
/// a drop shadow, etc.
///
/// [`Style`]: enum.Style.html
/// [`HSlider`]: ../../native/h_slider/struct.HSlider.html
/// [`Handle`]: https://docs.rs/iced/0.1.1/iced/widget/image/struct.Handle.html
#[derive(Debug, Clone)]
pub struct TextureStyle {
    /// colors of the top and bottom of the rail
    pub rail_colors: (Color, Color),
    /// the [`Handle`] to the image texture
    pub texture: image::Handle,
    /// the width of the handle, not including padding
    pub handle_width: u16,
    /// the texture padding around the handle bounding
    /// rectangle. This is useful when the texture is of a glowing handle or has
    /// a drop shadow, etc.
    pub texture_padding: Option<TexturePadding>,
}

/// A classic [`Style`] for an [`HSlider`], modeled after hardware sliders 
///
/// * `rail_colors` - colors of the top and bottom of the rail
/// * `handle` - a [`ClassicHandle`] defining the style of the handle
///
/// [`Style`]: enum.Style.html
/// [`HSlider`]: ../../native/h_slider/struct.HSlider.html
/// [`ClassicHandle`]: struct.ClassicHandle.html
#[derive(Debug, Clone)]
pub struct ClassicStyle {
    /// colors of the top and bottom of the rail
    pub rail_colors: (Color, Color),
    /// a `ClassicHandle` defining the style of the handle
    pub handle: ClassicHandle,
}

/// The [`ClassicStyle`] appearance of the handle of an [`HSlider`]
///
/// * `color` - background color
/// * `width` - width of the handle
/// * `notch_width` - width of the middle notch
/// * `notch_color` - color of the middle notch
/// * `border_radius` - radius of the background rectangle
/// * `border_width` - width of the background rectangle
/// * `border_color` - color of the background rectangle border
///
/// [`HSlider`]: ../../native/h_slider/struct.HSlider.html
/// [`ClassicStyle`]: struct.ClassicStyle.html
#[derive(Debug, Clone)]
pub struct ClassicHandle {
    /// background color
    pub color: Color,
    /// width of the handle
    pub width: u16,
    /// width of the middle notch
    pub notch_width: u16,
    /// color of the middle notch
    pub notch_color: Color,
    /// radius of the background rectangle
    pub border_radius: u16,
    /// width of the background rectangle
    pub border_width: u16,
    /// color of the background rectangle border
    pub border_color: Color,
}

/// A modern [`Style`] for an [`HSlider`]. It is composed of a background
/// rectangle and a rectangular handle.
///
/// * `back_empty_color` - color of an unfilled portion in the background
/// rectangle
/// * `back_filled_color` - color of a filled portion in the background
/// rectangle
/// * `border_color` - color of the background rectangle border
/// * `border_radius` - radius of the background rectangle
/// * `border_width` - width of the background rectangle border
/// * `handle_color` - color of the handle rectangle
/// * `handle_width` - width of the handle rectangle
/// * `handle_filled_gap` - width of the gap between the handle and the filled
/// portion of the background rectangle
///
/// [`Style`]: enum.Style.html
/// [`HSlider`]: ../../native/h_slider/struct.HSlider.html
#[derive(Debug, Clone, Copy)]
pub struct RectStyle {
    /// color of an unfilled portion in the background rectangle
    pub back_empty_color: Color,
    /// color of a filled portion in the background rectangle
    pub back_filled_color: Color,
    /// color of the background rectangle border
    pub border_color: Color,
    /// radius of the background rectangle
    pub border_radius: u16,
    /// width of the background rectangle border
    pub border_width: u16,
    /// color of the handle rectangle
    pub handle_color: Color,
    /// width of the handle rectangle
    pub handle_width: u16,
    /// width of the gap between the handle and the filled
    /// portion of the background rectangle
    pub handle_filled_gap: u16,
}

/// A modern [`Style`] for an [`HSlider`]. It is composed of a background
/// rectangle and a rectangular handle. It has different colors for left, right,
/// and center values.
///
/// * `back_left_empty_color` - color of an unfilled portion in the background
/// rectangle on the left side of the center
/// * `back_left_filled_color` - color of a filled portion in the background
/// rectangle on the left side of the center
/// * `back_right_empty_color` - color of an unfilled portion in the background
/// rectangle on the right side of the center
/// * `back_right_filled_color` - color of a filled portion in the background
/// rectangle on the right side of the center
/// * `border_color` - color of the background rectangle border
/// * `border_radius` - radius of the background rectangle
/// * `border_width` - width of the background rectangle border
/// * `handle_left_color` - color of the handle rectangle when it is on the
/// left side of the center
/// * `handle_right_color` - color of the handle rectangle when it is on the
/// right side of the center
/// * `handle_center_color` - color of the handle rectangle when it is in
/// the center
/// * `handle_width` - width of the handle rectangle
/// * `handle_filled_gap` - width of the gap between the handle and the filled
/// portion of the background rectangle
///
/// [`Style`]: enum.Style.html
/// [`HSlider`]: ../../native/h_slider/struct.HSlider.html
#[derive(Debug, Clone, Copy)]
pub struct RectBipolarStyle {
    /// color of an unfilled portion in the background
    /// rectangle on the left side of the center
    pub back_left_empty_color: Color,
    /// color of a filled portion in the background
    /// rectangle on the left side of the center
    pub back_left_filled_color: Color,
    /// color of an unfilled portion in the background
    /// rectangle on the right side of the center
    pub back_right_empty_color: Color,
    /// color of a filled portion in the background
    /// rectangle on the right side of the center
    pub back_right_filled_color: Color,
    /// color of the background rectangle border
    pub border_color: Color,
    /// radius of the background rectangle
    pub border_radius: u16,
    /// width of the background rectangle border
    pub border_width: u16,
    /// color of the handle rectangle when it is on the
    /// left side of the center
    pub handle_left_color: Color,
    /// color of the handle rectangle when it is on the
    /// right side of the center
    pub handle_right_color: Color,
    /// color of the handle rectangle when it is in the center
    pub handle_center_color: Color,
    /// width of the handle rectangle
    pub handle_width: u16,
    /// width of the gap between the handle and the filled
    /// portion of the background rectangle
    pub handle_filled_gap: u16,
}

/// A set of rules that dictate the style of an [`HSlider`].
///
/// [`HSlider`]: ../../native/h_slider/struct.HSlider.html
pub trait StyleSheet {
    /// Produces the style of an active [`HSlider`].
    ///
    /// [`HSlider`]: ../../native/h_slider/struct.HSlider.html
    fn active(&self) -> Style;

    /// Produces the style of a hovered [`HSlider`].
    ///
    /// [`HSlider`]: ../../native/h_slider/struct.HSlider.html
    fn hovered(&self) -> Style;

    /// Produces the style of an [`HSlider`] that is being dragged.
    ///
    /// [`HSlider`]: ../../native/h_slider/struct.HSlider.html
    fn dragging(&self) -> Style;
}

struct Default;

impl StyleSheet for Default {
    fn active(&self) -> Style {
        Style::Classic(
        ClassicStyle {
            rail_colors: ([0.56, 0.56, 0.56, 0.75].into(), Color::WHITE),
            handle: ClassicHandle {
                color: Color::from_rgb(0.97, 0.97, 0.97),
                width: 33,
                notch_width: 4,
                notch_color: Color::from_rgb(0.475, 0.475, 0.475),
                border_radius: 2,
                border_color: Color::from_rgb(0.51, 0.51, 0.51),
                border_width: 1,
            },
        }
        )
    }

    fn hovered(&self) -> Style {
        let active = self.active();
        if let Style::Classic(active) = self.active() {

        Style::Classic(
        ClassicStyle {
            handle: ClassicHandle {
                color: Color::from_rgb(0.93, 0.93, 0.93),
                ..active.handle
            },
            ..active
        })

        } else { active }
    }

    fn dragging(&self) -> Style {
        let active = self.active();
        if let Style::Classic(active) = self.active() {

        Style::Classic(
        ClassicStyle {
            handle: ClassicHandle {
                color: Color::from_rgb(0.92, 0.92, 0.92),
                ..active.handle
            },
            ..active
        })

        } else { active }
    }
}


impl std::default::Default for Box<dyn StyleSheet> {
    fn default() -> Self {
        Box::new(Default)
    }
}

impl<T> From<T> for Box<dyn StyleSheet>
where
    T: 'static + StyleSheet,
{
    fn from(style: T) -> Self {
        Box::new(style)
    }
}