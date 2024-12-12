// Copyright 2024 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

//! Flex properties can be set in Xilem.

#![expect(clippy::shadow_unrelated, reason = "Idiomatic for Xilem users")]

use std::env;
use masonry::widget::{CrossAxisAlignment, MainAxisAlignment};
use winit::error::EventLoopError;
use winit::window::Window;
use masonry::axis::Axis;
use masonry::dpi::LogicalSize;
use xilem::view::{button, flex, label, sized_box, FlexExt as _, checkbox, AnyFlexChild};
use xilem::{Color, EventLoop, WidgetView, Xilem};

struct FlexApp {
    flex_items: Vec<i32>,
    flex_values: Vec<f64>,
    include_fixed_wide: bool,
    include_fixed_narrow: bool,
    debug_paint: bool,
    show_flex_items: bool,
    axis: Axis,
    main_axis_align: MainAxisAlignment,
    cross_axis_align: CrossAxisAlignment,
}

fn main_axis_alignment_settings(data: &mut FlexApp) -> impl WidgetView<FlexApp> {
    flex((
        label("Main Axis Alignment"),
        checkbox(
            "Start",
            data.main_axis_align == MainAxisAlignment::Start,
            |data: &mut FlexApp, _checked| {
                data.main_axis_align = MainAxisAlignment::Start;
            },
        ),
        checkbox(
            "Center",
            data.main_axis_align == MainAxisAlignment::Center,
            |data: &mut FlexApp, _checked| {
                data.main_axis_align = MainAxisAlignment::Center;
            },
        ),
        checkbox(
            "End",
            data.main_axis_align == MainAxisAlignment::End,
            |data: &mut FlexApp, _checked| {
                data.main_axis_align = MainAxisAlignment::End;
            },
        ),
        checkbox(
            "SpaceBetween",
            data.main_axis_align == MainAxisAlignment::SpaceBetween,
            |data: &mut FlexApp, _checked| {
                data.main_axis_align = MainAxisAlignment::SpaceBetween;
            },
        ),
        checkbox(
            "SpaceEvenly",
            data.main_axis_align == MainAxisAlignment::SpaceEvenly,
            |data: &mut FlexApp, _checked| {
                data.main_axis_align = MainAxisAlignment::SpaceEvenly;
            },
        ),
        checkbox(
            "SpaceAround",
            data.main_axis_align == MainAxisAlignment::SpaceAround,
            |data: &mut FlexApp, _checked| {
                data.main_axis_align = MainAxisAlignment::SpaceAround;
            },
        ),
    ))
        .cross_axis_alignment(CrossAxisAlignment::Start)
}

fn cross_axis_alignment_settings(data: &mut FlexApp) -> impl WidgetView<FlexApp> {
    flex((
        label("Cross Axis Alignment"),
        checkbox(
            "Start",
            data.cross_axis_align == CrossAxisAlignment::Start,
            |data: &mut FlexApp, _checked| {
                data.cross_axis_align = CrossAxisAlignment::Start;
            },
        ),
        checkbox(
            "Center",
            data.cross_axis_align == CrossAxisAlignment::Center,
            |data: &mut FlexApp, _checked| {
                data.cross_axis_align = CrossAxisAlignment::Center;
            },
        ),
        checkbox(
            "End",
            data.cross_axis_align == CrossAxisAlignment::End,
            |data: &mut FlexApp, _checked| {
                data.cross_axis_align = CrossAxisAlignment::End;
            },
        ),
        checkbox(
            "Baseline",
            data.cross_axis_align == CrossAxisAlignment::Baseline,
            |data: &mut FlexApp, _checked| {
                data.cross_axis_align = CrossAxisAlignment::Baseline;
            },
        ),
        checkbox(
            "Fill",
            data.cross_axis_align == CrossAxisAlignment::Fill,
            |data: &mut FlexApp, _checked| {
                data.cross_axis_align = CrossAxisAlignment::Fill;
            },
        ),
    ))
        .cross_axis_alignment(CrossAxisAlignment::Start)
}

fn general_settings(data: &mut FlexApp) -> impl WidgetView<FlexApp> {
    flex((
        label("General"),
        checkbox("Show Flex Items", data.show_flex_items, |data: &mut FlexApp, checked| {
            data.show_flex_items = checked;
        }),
        checkbox("Vertical Axis", data.axis == Axis::Vertical, |data: &mut FlexApp, checked| {
            data.axis = if checked {
                Axis::Vertical
            } else {
                Axis::Horizontal
            };
        }),
        checkbox("Include Large Sized Box", data.include_fixed_wide, |data: &mut FlexApp, checked| {
            data.include_fixed_wide = checked;
        }),
        checkbox("Include Small Sized Box", data.include_fixed_wide, |data: &mut FlexApp, checked| {
            data.include_fixed_narrow = checked;
        }),
        checkbox("Debug Paint", data.debug_paint, |data: &mut FlexApp, checked| {
            data.debug_paint = checked;
            #[allow(unsafe_code)]
            unsafe {
                env::set_var(
                    "MASONRY_DEBUG_PAINT",
                    if checked {
                        "TRUE"
                    } else {
                        ""
                    },
                );
            }
        }),
    ))
        .cross_axis_alignment(CrossAxisAlignment::Start)
}

fn adjustable_flex(index: usize, data: &i32) -> AnyFlexChild<FlexApp> {
    sized_box(
        flex((
            button("-", move |data: &mut FlexApp| {
                if data.flex_items[index] > 1 {
                    data.flex_items[index] -= 1;
                }
            }),
            label(format!("{}", *data)),
            button("+", move |data: &mut FlexApp| {
                data.flex_items[index] += 1;
            }),
        ))
            .direction(Axis::Horizontal)
            .main_axis_alignment(MainAxisAlignment::Center)
    )
        .border(Color::YELLOW, 1)
        .flex(*data as f64)
        .into_any_flex()
}

fn playground(data: &mut FlexApp) -> impl WidgetView<FlexApp> {
    flex((
        data.include_fixed_wide.then(|| {
            sized_box(label("400 width flex 1")).width(400.0).border(Color::GREEN, 2).flex(1.0)
        }),
        data.show_flex_items.then(|| {
            (
                data.flex_items.iter_mut().enumerate().map(|(i, flex_item)| {
                    adjustable_flex(i, flex_item)
                }).collect::<Vec<_>>(),
                button("Add", |data: &mut FlexApp| {
                    data.flex_items.push(1);
                },
            )
        )}),
        data.include_fixed_narrow.then(|| {
            sized_box(label("50 width flex 1")).width(50.0).border(Color::GREEN, 2).flex(1.0)
        }),
    ))
        .direction(data.axis)
        .main_axis_alignment(data.main_axis_align)
        .cross_axis_alignment(data.cross_axis_align)
        .must_fill_major_axis(true)
}

fn app_logic(data: &mut FlexApp) -> impl WidgetView<FlexApp> {
    flex((
        sized_box(
            flex((
                label("Settings"),
                flex((
                    general_settings(data),
                    main_axis_alignment_settings(data),
                    cross_axis_alignment_settings(data),
                ))
                    .direction(Axis::Horizontal)
                    .gap(20.0)
                    .cross_axis_alignment(CrossAxisAlignment::Start),
            )),
        )
            .border(Color::rgb8(100, 100, 100), 5)
            .padding(10.0),
        sized_box(
            playground(data),
        )
            .background(Color::rgb8(30, 30, 30))
            .flex(1.0),



    ))
        .direction(Axis::Vertical)
        .cross_axis_alignment(CrossAxisAlignment::Fill)
        .main_axis_alignment(MainAxisAlignment::Start)
}

fn main() -> Result<(), EventLoopError> {
    let state = FlexApp{
        flex_values: Vec::new(),
        include_fixed_wide: true,
        include_fixed_narrow: true,
        debug_paint: false,
        show_flex_items: true,
        flex_items: vec![1],
        axis: Axis::Horizontal,
        main_axis_align: MainAxisAlignment::Start,
        cross_axis_align: CrossAxisAlignment::Start,
    };
    let app = Xilem::new(state, app_logic);

    let min_window_size = LogicalSize::new(100., 100.);
    let window_size = LogicalSize::new(900., 600.);
    let window_attributes = Window::default_attributes()
        .with_title("Flex")
        .with_resizable(true)
        .with_min_inner_size(min_window_size)
        .with_inner_size(window_size);
    app.run_windowed_in(EventLoop::with_user_event(), window_attributes)?;
    Ok(())
}
