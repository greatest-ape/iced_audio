//! `iced_graphics` renderer for tick marks for bar meters

use crate::core::Normal;
use crate::native::tick_marks;
use crate::style::tick_marks::{Placement, Shape, Style};
use iced_graphics::Primitive;
use iced_native::{Background, Color, Rectangle};

fn draw_horizontal_lines(
    primitives: &mut Vec<Primitive>,
    tick_marks: &[Normal],
    bounds_x: f32,
    bounds_width: f32,
    y: f32,
    width: u16,
    length: u16,
    color: Color,
    inverse: bool,
) {
    let start_x = bounds_x - (f32::from(width) / 2.0);
    let back_color = Background::Color(color);

    if inverse {
        for tick_mark in tick_marks {
            primitives.push(Primitive::Quad {
                bounds: Rectangle {
                    x: (start_x + tick_mark.scale_inv(bounds_width)).round(),
                    y,
                    width: f32::from(width),
                    height: f32::from(length),
                },
                background: back_color,
                border_radius: 0,
                border_width: 0,
                border_color: Color::TRANSPARENT,
            });
        }
    } else {
        for tick_mark in tick_marks {
            primitives.push(Primitive::Quad {
                bounds: Rectangle {
                    x: (start_x + tick_mark.scale(bounds_width)).round(),
                    y,
                    width: f32::from(width),
                    height: f32::from(length),
                },
                background: back_color,
                border_radius: 0,
                border_width: 0,
                border_color: Color::TRANSPARENT,
            });
        }
    }
}

fn draw_horizontal_circles(
    primitives: &mut Vec<Primitive>,
    tick_marks: &[Normal],
    bounds_x: f32,
    bounds_width: f32,
    y: f32,
    diameter: u16,
    color: Color,
    inverse: bool,
) {
    let diameter = f32::from(diameter);
    let radius = (diameter / 2.0).round() as u16;
    let start_x = bounds_x - f32::from(radius);
    let back_color = Background::Color(color);

    if inverse {
        for tick_mark in tick_marks {
            primitives.push(Primitive::Quad {
                bounds: Rectangle {
                    x: (start_x + tick_mark.scale_inv(bounds_width)).round(),
                    y,
                    width: diameter,
                    height: diameter,
                },
                background: back_color,
                border_radius: radius,
                border_width: 0,
                border_color: Color::TRANSPARENT,
            });
        }
    } else {
        for tick_mark in tick_marks {
            primitives.push(Primitive::Quad {
                bounds: Rectangle {
                    x: (start_x + tick_mark.scale(bounds_width)).round(),
                    y,
                    width: diameter,
                    height: diameter,
                },
                background: back_color,
                border_radius: radius,
                border_width: 0,
                border_color: Color::TRANSPARENT,
            });
        }
    }
}

#[inline]
fn draw_horizontal_top_aligned_tier(
    primitives: &mut Vec<Primitive>,
    bounds: &Rectangle,
    y: f32,
    tick_marks: Option<&Vec<Normal>>,
    shape: &Option<Shape>,
    inverse: bool,
) {
    if let Some(tick_marks) = tick_marks {
        if let Some(shape) = &shape {
            match shape {
                Shape::Line {
                    length,
                    width,
                    color,
                } => {
                    draw_horizontal_lines(
                        primitives,
                        tick_marks,
                        bounds.x,
                        bounds.width,
                        y,
                        *width,
                        *length,
                        *color,
                        inverse,
                    );
                }
                Shape::Circle { diameter, color } => {
                    draw_horizontal_circles(
                        primitives,
                        tick_marks,
                        bounds.x,
                        bounds.width,
                        y,
                        *diameter,
                        *color,
                        inverse,
                    );
                }
            }
        }
    }
}

fn draw_horizontal_top_aligned(
    primitives: &mut Vec<Primitive>,
    bounds: &Rectangle,
    y: f32,
    tick_marks: &tick_marks::Group,
    style: &Style,
    inverse: bool,
) {
    draw_horizontal_top_aligned_tier(
        primitives,
        bounds,
        y,
        tick_marks.tier_1(),
        &style.tier_1,
        inverse,
    );
    draw_horizontal_top_aligned_tier(
        primitives,
        bounds,
        y,
        tick_marks.tier_2(),
        &style.tier_2,
        inverse,
    );
    draw_horizontal_top_aligned_tier(
        primitives,
        bounds,
        y,
        tick_marks.tier_3(),
        &style.tier_3,
        inverse,
    );
}

#[inline]
fn draw_horizontal_bottom_aligned_tier(
    primitives: &mut Vec<Primitive>,
    bounds: &Rectangle,
    y: f32,
    tick_marks: Option<&Vec<Normal>>,
    shape: &Option<Shape>,
    inverse: bool,
) {
    if let Some(tick_marks) = tick_marks {
        if let Some(shape) = &shape {
            match shape {
                Shape::Line {
                    length,
                    width,
                    color,
                } => {
                    draw_horizontal_lines(
                        primitives,
                        tick_marks,
                        bounds.x,
                        bounds.width,
                        y - f32::from(*length),
                        *width,
                        *length,
                        *color,
                        inverse,
                    );
                }
                Shape::Circle { diameter, color } => {
                    draw_horizontal_circles(
                        primitives,
                        tick_marks,
                        bounds.x,
                        bounds.width,
                        y - f32::from(*diameter),
                        *diameter,
                        *color,
                        inverse,
                    );
                }
            }
        }
    }
}

fn draw_horizontal_bottom_aligned(
    primitives: &mut Vec<Primitive>,
    bounds: &Rectangle,
    y: f32,
    tick_marks: &tick_marks::Group,
    style: &Style,
    inverse: bool,
) {
    draw_horizontal_bottom_aligned_tier(
        primitives,
        bounds,
        y,
        tick_marks.tier_1(),
        &style.tier_1,
        inverse,
    );
    draw_horizontal_bottom_aligned_tier(
        primitives,
        bounds,
        y,
        tick_marks.tier_2(),
        &style.tier_2,
        inverse,
    );
    draw_horizontal_bottom_aligned_tier(
        primitives,
        bounds,
        y,
        tick_marks.tier_3(),
        &style.tier_3,
        inverse,
    );
}

#[inline]
fn draw_horizontal_center_aligned_tier(
    primitives: &mut Vec<Primitive>,
    bounds: &Rectangle,
    y: f32,
    tick_marks: Option<&Vec<Normal>>,
    shape: &Option<Shape>,
    fill_length: bool,
    inverse: bool,
) {
    if let Some(tick_marks) = tick_marks {
        if let Some(shape) = &shape {
            match shape {
                Shape::Line {
                    length,
                    width,
                    color,
                } => {
                    let (y, length) = if fill_length {
                        (
                            bounds.y + f32::from(*length),
                            (bounds.height - (f32::from(*length) * 2.0)) as u16,
                        )
                    } else {
                        ((y - (f32::from(*length) / 2.0)).round(), *length)
                    };

                    draw_horizontal_lines(
                        primitives,
                        tick_marks,
                        bounds.x,
                        bounds.width,
                        y,
                        *width,
                        length,
                        *color,
                        inverse,
                    );
                }
                Shape::Circle { diameter, color } => {
                    let (y, diameter) = if fill_length {
                        (
                            bounds.y + f32::from(*diameter),
                            (bounds.height - (f32::from(*diameter) * 2.0))
                                as u16,
                        )
                    } else {
                        ((y - (f32::from(*diameter) / 2.0)).round(), *diameter)
                    };

                    draw_horizontal_circles(
                        primitives,
                        tick_marks,
                        bounds.x,
                        bounds.width,
                        y,
                        diameter,
                        *color,
                        inverse,
                    );
                }
            }
        }
    }
}

fn draw_horizontal_center_aligned(
    primitives: &mut Vec<Primitive>,
    bounds: &Rectangle,
    y: f32,
    tick_marks: &tick_marks::Group,
    style: &Style,
    fill_length: bool,
    inverse: bool,
) {
    draw_horizontal_center_aligned_tier(
        primitives,
        bounds,
        y,
        tick_marks.tier_1(),
        &style.tier_1,
        fill_length,
        inverse,
    );
    draw_horizontal_center_aligned_tier(
        primitives,
        bounds,
        y,
        tick_marks.tier_2(),
        &style.tier_2,
        fill_length,
        inverse,
    );
    draw_horizontal_center_aligned_tier(
        primitives,
        bounds,
        y,
        tick_marks.tier_3(),
        &style.tier_3,
        fill_length,
        inverse,
    );
}

#[inline]
fn draw_horizontal_center_aligned_split_tier(
    primitives: &mut Vec<Primitive>,
    bounds: &Rectangle,
    y: f32,
    tick_marks: Option<&Vec<Normal>>,
    shape: &Option<Shape>,
    fill_length: bool,
    gap: f32,
    inverse: bool,
) {
    if let Some(tick_marks) = tick_marks {
        if let Some(shape) = &shape {
            match shape {
                Shape::Line {
                    length,
                    width,
                    color,
                } => {
                    let (left_y, length) = if fill_length {
                        let length = (f32::from(*length)
                            + ((bounds.height + gap) / 2.0))
                            .round();
                        ((y - length - (gap / 2.0)).round(), length as u16)
                    } else {
                        (
                            (y - f32::from(*length) - (gap / 2.0)).round(),
                            *length,
                        )
                    };

                    let right_y = (y + (gap / 2.0)).round();

                    draw_horizontal_lines(
                        primitives,
                        tick_marks,
                        bounds.x,
                        bounds.width,
                        left_y,
                        *width,
                        length,
                        *color,
                        inverse,
                    );
                    draw_horizontal_lines(
                        primitives,
                        tick_marks,
                        bounds.x,
                        bounds.width,
                        right_y,
                        *width,
                        length,
                        *color,
                        inverse,
                    );
                }
                Shape::Circle { diameter, color } => {
                    let (left_y, diameter) = if fill_length {
                        (
                            bounds.y - f32::from(*diameter),
                            (f32::from(*diameter)
                                + ((bounds.height + gap) / 2.0))
                                .round() as u16,
                        )
                    } else {
                        (
                            (y - f32::from(*diameter) - (gap / 2.0)).round(),
                            *diameter,
                        )
                    };

                    let right_y = (y + (gap / 2.0)).round();

                    draw_horizontal_circles(
                        primitives,
                        tick_marks,
                        bounds.x,
                        bounds.width,
                        left_y,
                        diameter,
                        *color,
                        inverse,
                    );
                    draw_horizontal_circles(
                        primitives,
                        tick_marks,
                        bounds.x,
                        bounds.width,
                        right_y,
                        diameter,
                        *color,
                        inverse,
                    );
                }
            }
        }
    }
}

fn draw_horizontal_center_aligned_split(
    primitives: &mut Vec<Primitive>,
    bounds: &Rectangle,
    y: f32,
    tick_marks: &tick_marks::Group,
    style: &Style,
    fill_length: bool,
    gap: f32,
    inverse: bool,
) {
    draw_horizontal_center_aligned_split_tier(
        primitives,
        bounds,
        y,
        tick_marks.tier_1(),
        &style.tier_1,
        fill_length,
        gap,
        inverse,
    );
    draw_horizontal_center_aligned_split_tier(
        primitives,
        bounds,
        y,
        tick_marks.tier_2(),
        &style.tier_2,
        fill_length,
        gap,
        inverse,
    );
    draw_horizontal_center_aligned_split_tier(
        primitives,
        bounds,
        y,
        tick_marks.tier_3(),
        &style.tier_3,
        fill_length,
        gap,
        inverse,
    );
}

pub fn draw_horizontal_tick_marks(
    bounds: &Rectangle,
    tick_marks: &tick_marks::Group,
    style: &Style,
    placement: Placement,
    inverse: bool,
) -> Primitive {
    let mut primitives: Vec<Primitive> =
        Vec::with_capacity(tick_marks.len() * 2);

    match placement {
        Placement::BothSides { offset, inside } => {
            if inside {
                draw_horizontal_top_aligned(
                    &mut primitives,
                    bounds,
                    bounds.x + f32::from(offset),
                    tick_marks,
                    style,
                    inverse,
                );
                draw_horizontal_bottom_aligned(
                    &mut primitives,
                    bounds,
                    bounds.x + bounds.width - f32::from(offset),
                    tick_marks,
                    style,
                    inverse,
                );
            } else {
                draw_horizontal_bottom_aligned(
                    &mut primitives,
                    bounds,
                    bounds.x - f32::from(offset),
                    tick_marks,
                    style,
                    inverse,
                );
                draw_horizontal_top_aligned(
                    &mut primitives,
                    bounds,
                    bounds.x + bounds.width + f32::from(offset),
                    tick_marks,
                    style,
                    inverse,
                );
            }
        }
        Placement::LeftOrTop { offset, inside } => {
            if inside {
                draw_horizontal_top_aligned(
                    &mut primitives,
                    bounds,
                    bounds.x + f32::from(offset),
                    tick_marks,
                    style,
                    inverse,
                );
            } else {
                draw_horizontal_bottom_aligned(
                    &mut primitives,
                    bounds,
                    bounds.x - f32::from(offset),
                    tick_marks,
                    style,
                    inverse,
                );
            }
        }
        Placement::RightOrBottom { offset, inside } => {
            if inside {
                draw_horizontal_bottom_aligned(
                    &mut primitives,
                    bounds,
                    bounds.x + bounds.width - f32::from(offset),
                    tick_marks,
                    style,
                    inverse,
                );
            } else {
                draw_horizontal_top_aligned(
                    &mut primitives,
                    bounds,
                    bounds.x + bounds.width + f32::from(offset),
                    tick_marks,
                    style,
                    inverse,
                );
            }
        }
        Placement::Center { fill_length } => {
            draw_horizontal_center_aligned(
                &mut primitives,
                bounds,
                bounds.center_y().round(),
                tick_marks,
                style,
                fill_length,
                inverse,
            );
        }
        Placement::CenterSplit { fill_length, gap } => {
            draw_horizontal_center_aligned_split(
                &mut primitives,
                bounds,
                bounds.center_y().round(),
                tick_marks,
                style,
                fill_length,
                f32::from(gap),
                inverse,
            );
        }
    }

    Primitive::Group { primitives }
}
