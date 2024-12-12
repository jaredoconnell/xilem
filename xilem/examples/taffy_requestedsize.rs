// Copyright 2024 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

use taffy::Display::Flex;
use taffy::{AlignItems, FlexDirection, JustifyContent};
use masonry::Color;
use winit::error::EventLoopError;
use xilem::{
    view::sized_box,
    EventLoop, WidgetView, Xilem,
};
use xilem::view::{grid, prose, taffy_layout, TaffyExt};
fn app_logic(data: &mut i32) -> impl WidgetView<i32> {
    taffy_layout((
        sized_box(prose("a fixed-width box")).width(150.0).border(Color::rgb8(255, 40, 50), 5.0),
        sized_box(prose("a 50% width box")).border(Color::rgb8(40, 255, 60), 5.0).with_taffy_style(taffy::Style{
            size: taffy::Size {
                width: taffy::Dimension::Percent(0.5),
                height: taffy::Dimension::Auto,
            },
            ..taffy::Style::default()
        }),
        prose("A small string"),
        sized_box(
            grid((
                taffy_layout(
                    sized_box(
                        prose("a 50% width box within a grid of 2. This should be 50% of the parent, not 50% of the cell.")
                    ).border(Color::rgb8(40, 255, 60), 5.0),
                    taffy::Style{
                        size: taffy::Size {
                            width: taffy::Dimension::Percent(0.5),
                            height: taffy::Dimension::Auto,
                        },
                        ..taffy::Style::default()
                    },
                )
            ), 2, 1),
        ).height(24.0),
        prose("A large string that is definitely larger than the fixed-size box"),
    ), taffy::Style{
        /*size: taffy::Size {
            width: taffy::Dimension::Length(50.0),
            height: taffy::Dimension::Auto,
        },*/
        flex_direction: FlexDirection::Column,
        display: Flex,
        justify_content: Option::from(JustifyContent::Center),
        align_items: Option::from(AlignItems::Center),
        gap: taffy::Size{
            width: taffy::LengthPercentage::Length(15.0),
            height: taffy::LengthPercentage::Length(15.0),
        },
        ..taffy::Style::default()
    })
}

fn main() -> Result<(), EventLoopError> {
    let app = Xilem::new(0, app_logic);
    app.run_windowed(EventLoop::with_user_event(), "Taffy RequestedSize".into())?;
    Ok(())
}
