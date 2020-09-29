use crate::native::text_marks;
use crate::style::text_marks::{Placement, Style};

use iced_graphics::Primitive;
use iced_native::{Align, HorizontalAlignment, Rectangle, VerticalAlignment};

fn draw_aligned(
    primitives: &mut Vec<Primitive>,
    bounds: &Rectangle,
    x: f32,
    text_marks: &text_marks::Group,
    style: &Style,
    inverse: bool,
    align: HorizontalAlignment,
) {
    let color = style.color;
    let font = style.font;
    let text_size = f32::from(style.text_size);
    let text_bounds_width = f32::from(style.bounds_width);
    let text_bounds_height = f32::from(style.bounds_height);

    let x = (x + style.offset.x).round();
    let start_y = bounds.y + style.offset.y;

    if inverse {
        for text_mark in &text_marks.group {
            primitives.push(Primitive::Text {
                content: text_mark.1.clone(),
                size: text_size,
                bounds: Rectangle {
                    x,
                    y: (start_y + (text_mark.0.scale(bounds.height))).round(),
                    width: text_bounds_width,
                    height: text_bounds_height,
                },
                color,
                font,
                horizontal_alignment: align,
                vertical_alignment: VerticalAlignment::Center,
            });
        }
    } else {
        for text_mark in &text_marks.group {
            primitives.push(Primitive::Text {
                content: text_mark.1.clone(),
                size: text_size,
                bounds: Rectangle {
                    x,
                    y: (start_y + (text_mark.0.scale_inv(bounds.height)))
                        .round(),
                    width: text_bounds_width,
                    height: text_bounds_height,
                },
                color,
                font,
                horizontal_alignment: align,
                vertical_alignment: VerticalAlignment::Center,
            });
        }
    }
}

pub fn draw_vertical_text_marks(
    bounds: &Rectangle,
    text_marks: &text_marks::Group,
    style: &Style,
    inverse: bool,
) -> Primitive {
    let primitives = match style.placement {
        Placement::BothSides { inside } => {
            let mut primitives: Vec<Primitive> =
                Vec::with_capacity(text_marks.group.len() * 2);

            if inside {
                draw_aligned(
                    &mut primitives,
                    bounds,
                    bounds.x,
                    text_marks,
                    style,
                    inverse,
                    HorizontalAlignment::Left,
                );
                draw_aligned(
                    &mut primitives,
                    bounds,
                    bounds.x + bounds.width,
                    text_marks,
                    style,
                    inverse,
                    HorizontalAlignment::Right,
                );
            } else {
                draw_aligned(
                    &mut primitives,
                    bounds,
                    bounds.x,
                    text_marks,
                    style,
                    inverse,
                    HorizontalAlignment::Right,
                );
                draw_aligned(
                    &mut primitives,
                    bounds,
                    bounds.x + bounds.width,
                    text_marks,
                    style,
                    inverse,
                    HorizontalAlignment::Left,
                );
            }

            primitives
        }
        Placement::LeftOrTop { inside } => {
            let mut primitives: Vec<Primitive> =
                Vec::with_capacity(text_marks.group.len());

            if inside {
                draw_aligned(
                    &mut primitives,
                    bounds,
                    bounds.x,
                    text_marks,
                    style,
                    inverse,
                    HorizontalAlignment::Left,
                );
            } else {
                draw_aligned(
                    &mut primitives,
                    bounds,
                    bounds.x,
                    text_marks,
                    style,
                    inverse,
                    HorizontalAlignment::Right,
                );
            }

            primitives
        }
        Placement::RightOrBottom { inside } => {
            let mut primitives: Vec<Primitive> =
                Vec::with_capacity(text_marks.group.len());

            if inside {
                draw_aligned(
                    &mut primitives,
                    bounds,
                    bounds.x + bounds.width,
                    text_marks,
                    style,
                    inverse,
                    HorizontalAlignment::Right,
                );
            } else {
                draw_aligned(
                    &mut primitives,
                    bounds,
                    bounds.x + bounds.width,
                    text_marks,
                    style,
                    inverse,
                    HorizontalAlignment::Left,
                );
            }

            primitives
        }
        Placement::Center { align } => {
            let mut primitives: Vec<Primitive> =
                Vec::with_capacity(text_marks.group.len());

            match align {
                Align::Start => {
                    draw_aligned(
                        &mut primitives,
                        bounds,
                        bounds.center_x(),
                        text_marks,
                        style,
                        inverse,
                        HorizontalAlignment::Left,
                    );
                }
                Align::End => {
                    draw_aligned(
                        &mut primitives,
                        bounds,
                        bounds.center_x(),
                        text_marks,
                        style,
                        inverse,
                        HorizontalAlignment::Right,
                    );
                }
                Align::Center => {
                    draw_aligned(
                        &mut primitives,
                        bounds,
                        bounds.center_x(),
                        text_marks,
                        style,
                        inverse,
                        HorizontalAlignment::Center,
                    );
                }
            }

            primitives
        }
    };

    Primitive::Group { primitives }
}
