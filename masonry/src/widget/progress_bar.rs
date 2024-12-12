// Copyright 2019 the Xilem Authors and the Druid Authors
// SPDX-License-Identifier: Apache-2.0

//! A progress bar widget.

use accesskit::{Node, Role};
use smallvec::{smallvec, SmallVec};
use tracing::{trace_span, Span};
use vello::Scene;

use crate::kurbo::Size;
use crate::paint_scene_helpers::{fill_lin_gradient, stroke, UnitPoint};
use crate::text::ArcStr;
use crate::widget::{ContentFill, WidgetMut};

use crate::{
    theme, AccessCtx, AccessEvent, BoxConstraints, EventCtx, LayoutCtx, PaintCtx, Point,
    PointerEvent, QueryCtx, RegisterCtx, TextEvent, Update, UpdateCtx, Widget, WidgetId,
};
use crate::axis::Axis;
use crate::biaxial::BiAxial;

const DEFAULT_WIDTH: f64 = 400.;

use super::{Label, LineBreaking, WidgetPod};

/// A progress bar.
pub struct ProgressBar {
    /// A value in the range `[0, 1]` inclusive, where 0 is 0% and 1 is 100% complete.
    ///
    /// `None` variant can be used to show a progress bar without a percentage.
    /// It is also used if an invalid float (outside of [0, 1]) is passed.
    progress: Option<f64>,
    label: WidgetPod<Label>,
}

impl ProgressBar {
    /// Create a new `ProgressBar`.
    ///
    /// `progress` is a number between 0 and 1 inclusive. If it is `NaN`, then an
    /// indefinite progress bar will be shown.
    /// Otherwise, the input will be clamped to [0, 1].
    pub fn new(mut progress: Option<f64>) -> Self {
        clamp_progress(&mut progress);
        let label = WidgetPod::new(
            Label::new(Self::value(progress)).with_line_break_mode(LineBreaking::Overflow),
        );
        Self { progress, label }
    }

    fn value_accessibility(&self) -> Box<str> {
        if let Some(value) = self.progress {
            format!("{:.0}%", value * 100.).into()
        } else {
            "progress unspecified".into()
        }
    }

    fn value(progress: Option<f64>) -> ArcStr {
        if let Some(value) = progress {
            format!("{:.0}%", value * 100.).into()
        } else {
            "".into()
        }
    }
}

// --- MARK: WIDGETMUT ---
impl ProgressBar {
    pub fn set_progress(this: &mut WidgetMut<'_, Self>, mut progress: Option<f64>) {
        clamp_progress(&mut progress);
        let progress_changed = this.widget.progress != progress;
        if progress_changed {
            this.widget.progress = progress;
            let mut label = this.ctx.get_mut(&mut this.widget.label);
            Label::set_text(&mut label, Self::value(progress));
        }
        this.ctx.request_layout();
        this.ctx.request_render();
    }
}

/// Helper to ensure progress is either a number between [0, 1] inclusive, or `None`.
///
/// NaNs are converted to `None`.
fn clamp_progress(progress: &mut Option<f64>) {
    if let Some(value) = progress {
        if value.is_nan() {
            *progress = None;
        } else {
            *progress = Some(value.clamp(0., 1.));
        }
    }
}

// --- MARK: IMPL WIDGET ---
impl Widget for ProgressBar {
    fn on_pointer_event(&mut self, _ctx: &mut EventCtx, _event: &PointerEvent) {}

    fn on_text_event(&mut self, _ctx: &mut EventCtx, _event: &TextEvent) {}

    fn on_access_event(&mut self, _ctx: &mut EventCtx, _event: &AccessEvent) {}

    fn register_children(&mut self, ctx: &mut RegisterCtx) {
        ctx.register_child(&mut self.label);
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _event: &Update) {}

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints) -> Size {
        const DEFAULT_WIDTH: f64 = 400.;
        // TODO: Fix centering
        let label_size = ctx.run_layout(&mut self.label, bc);
        let desired_size = Size::new(
            DEFAULT_WIDTH.max(label_size.width),
            crate::theme::BASIC_WIDGET_HEIGHT.max(label_size.height),
        );
        let final_size = bc.constrain(desired_size);

        // center text
        let text_pos = Point::new(
            ((final_size.width - label_size.width) * 0.5).max(0.),
            ((final_size.height - label_size.height) * 0.5).max(0.),
        );
        ctx.place_child(&mut self.label, text_pos);
        final_size
    }

    fn measure(&mut self, ctx: &mut LayoutCtx, axis: Axis, fill: &BiAxial<ContentFill>) -> f64 {
        let label_size = ctx.run_measure(&mut self.label, axis, fill);
        let min_size = match axis {
            Axis::Horizontal => DEFAULT_WIDTH,
            Axis::Vertical => theme::BASIC_WIDGET_HEIGHT,
        };
        let widget_size = label_size.max(min_size);
        match fill.value_for_axis(axis) {
            ContentFill::Max => widget_size,
            ContentFill::Min => label_size.min(min_size),
            ContentFill::Constrain(constrained_size) => widget_size.min(constrained_size),
            ContentFill::MaxStretch => f64::INFINITY,
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, scene: &mut Scene) {
        let border_width = 1.;

        let rect = ctx
            .size()
            .to_rect()
            .inset(-border_width / 2.)
            .to_rounded_rect(2.);

        fill_lin_gradient(
            scene,
            &rect,
            [theme::BACKGROUND_LIGHT, theme::BACKGROUND_DARK],
            UnitPoint::TOP,
            UnitPoint::BOTTOM,
        );

        stroke(scene, &rect, theme::BORDER_DARK, border_width);

        let progress_rect_size = Size::new(
            ctx.size().width * self.progress.unwrap_or(1.),
            ctx.size().height,
        );
        let progress_rect = progress_rect_size
            .to_rect()
            .inset(-border_width / 2.)
            .to_rounded_rect(2.);

        fill_lin_gradient(
            scene,
            &progress_rect,
            [theme::PRIMARY_LIGHT, theme::PRIMARY_DARK],
            UnitPoint::TOP,
            UnitPoint::BOTTOM,
        );
        stroke(scene, &progress_rect, theme::BORDER_DARK, border_width);
    }

    fn accessibility_role(&self) -> Role {
        Role::ProgressIndicator
    }

    fn accessibility(&mut self, _ctx: &mut AccessCtx, node: &mut Node) {
        node.set_value(self.value_accessibility());
        if let Some(value) = self.progress {
            node.set_numeric_value(value * 100.0);
        }
    }

    fn children_ids(&self) -> SmallVec<[WidgetId; 16]> {
        smallvec![self.label.id()]
    }

    fn make_trace_span(&self, ctx: &QueryCtx<'_>) -> Span {
        trace_span!("ProgressBar", id = ctx.widget_id().trace())
    }

    fn get_debug_text(&self) -> Option<String> {
        Some(self.value_accessibility().into())
    }
}

// --- MARK: TESTS ---
#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use super::*;
    use crate::assert_render_snapshot;
    use crate::testing::{widget_ids, TestHarness, TestWidgetExt};

    #[test]
    fn indeterminate_progressbar() {
        let [progressbar_id] = widget_ids();
        let widget = ProgressBar::new(None).with_id(progressbar_id);

        let mut harness = TestHarness::create(widget);

        assert_debug_snapshot!(harness.root_widget());
        assert_render_snapshot!(harness, "indeterminate_progressbar");
    }

    #[test]
    fn _0_percent_progressbar() {
        let [_0percent] = widget_ids();

        let widget = ProgressBar::new(Some(0.)).with_id(_0percent);
        let mut harness = TestHarness::create(widget);
        assert_debug_snapshot!(harness.root_widget());
        assert_render_snapshot!(harness, "0_percent_progressbar");
    }

    #[test]
    fn _25_percent_progressbar() {
        let [_25percent] = widget_ids();

        let widget = ProgressBar::new(Some(0.25)).with_id(_25percent);
        let mut harness = TestHarness::create(widget);
        assert_debug_snapshot!(harness.root_widget());
        assert_render_snapshot!(harness, "25_percent_progressbar");
    }

    #[test]
    fn _50_percent_progressbar() {
        let [_50percent] = widget_ids();

        let widget = ProgressBar::new(Some(0.5)).with_id(_50percent);
        let mut harness = TestHarness::create(widget);
        assert_debug_snapshot!(harness.root_widget());
        assert_render_snapshot!(harness, "50_percent_progressbar");
    }

    #[test]
    fn _75_percent_progressbar() {
        let [_75percent] = widget_ids();

        let widget = ProgressBar::new(Some(0.75)).with_id(_75percent);
        let mut harness = TestHarness::create(widget);
        assert_debug_snapshot!(harness.root_widget());
        assert_render_snapshot!(harness, "75_percent_progressbar");
    }

    #[test]
    fn _100_percent_progressbar() {
        let [_100percent] = widget_ids();

        let widget = ProgressBar::new(Some(1.)).with_id(_100percent);
        let mut harness = TestHarness::create(widget);
        assert_debug_snapshot!(harness.root_widget());
        assert_render_snapshot!(harness, "100_percent_progressbar");
    }

    #[test]
    fn edit_progressbar() {
        let image_1 = {
            let bar = ProgressBar::new(Some(0.5));

            let mut harness = TestHarness::create_with_size(bar, Size::new(60.0, 20.0));

            harness.render()
        };

        let image_2 = {
            let bar = ProgressBar::new(None);

            let mut harness = TestHarness::create_with_size(bar, Size::new(60.0, 20.0));

            harness.edit_root_widget(|mut label| {
                let mut bar = label.downcast::<ProgressBar>();
                ProgressBar::set_progress(&mut bar, Some(0.5));
            });

            harness.render()
        };

        // We don't use assert_eq because we don't want rich assert
        assert!(image_1 == image_2);
    }
}
