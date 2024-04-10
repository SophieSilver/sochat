use egui::{
    scroll_area::ScrollAreaOutput, Color32, Context, Frame, InnerResponse, Layout, ScrollArea,
    TextEdit, TopBottomPanel, Ui,
};

use crate::{actions, state::AppState};

/// Bottom panel, but it automatically sizes up to the size, returned by the closure passed to `show`
#[derive(Debug, Clone)]
#[must_use = "You should call .show()"]
struct SizedBottomPanel {
    id: egui::Id,
    frame: Option<Frame>,
}

impl SizedBottomPanel {
    fn new(id: impl Into<egui::Id>) -> Self {
        Self {
            id: id.into(),
            frame: None,
        }
    }

    fn frame(self, frame: Frame) -> Self {
        Self {
            frame: Some(frame),
            ..self
        }
    }

    fn show<R>(
        self,
        ctx: &Context,
        add_contents: impl FnOnce(&mut Ui) -> (R, f32),
    ) -> InnerResponse<R> {
        let height_id = self.id.with("current_height");
        let current_height = ctx
            .data(|data| data.get_temp::<f32>(height_id))
            .unwrap_or_default();

        let frame_margins = self
            .frame
            .map(|f| {
                let total_margin = f.total_margin();
                total_margin.bottom + total_margin.top
            })
            .unwrap_or(0.0);

        let panel = TopBottomPanel::bottom(self.id)
            .frame(Frame::none())
            .resizable(false)
            .show_separator_line(false)
            // using min and max height here, instead of exact height, because exact height causes jittering for some reason
            .min_height(0.0)
            .max_height(current_height + frame_margins);

        let InnerResponse {
            inner: (inner, height),
            response,
        } = panel.show(ctx, |ui| match self.frame {
            Some(frame) => frame.show(ui, add_contents).inner,
            None => add_contents(ui),
        });

        ctx.data_mut(|data| data.insert_temp(height_id, height));

        InnerResponse { inner, response }
    }
}

/// Vertical ScrollArea that automatically adjusts it's `min_scrolled_height` with
#[derive(Debug, Clone)]
#[must_use = "You should call .show()"]
struct SizedVerticalScrollArea {
    id: egui::Id,
}

impl SizedVerticalScrollArea {
    fn new(id: impl Into<egui::Id>) -> Self {
        Self { id: id.into() }
    }

    fn show<R>(
        self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> (R, f32),
    ) -> ScrollAreaOutput<R> {
        let height_id = self.id.with("current_height");
        let current_height = ui
            .ctx()
            .data(|data| data.get_temp::<f32>(height_id))
            .unwrap_or_default();

        let scroll_area = ScrollArea::vertical()
            .min_scrolled_height(current_height)
            .stick_to_bottom(true);

        let ScrollAreaOutput {
            inner: (inner, height),
            id,
            state,
            content_size,
            inner_rect,
        } = scroll_area.show(ui, add_contents);

        ui.ctx()
            .data_mut(|data| data.insert_temp(height_id, height));

        ScrollAreaOutput {
            inner,
            id,
            state,
            content_size,
            inner_rect,
        }
    }
}

pub fn show(state: &AppState, ctx: &Context) {
    let frame = Frame::side_top_panel(&ctx.style())
        .inner_margin(10.0)
        .fill(Color32::from_gray(32));

    SizedBottomPanel::new("TextBottomPanel")
        .frame(frame)
        .show(ctx, |ui| {
            let desired_height = text_input(state, ui);

            ((), desired_height)
        });
}

fn text_input(state: &AppState, ui: &mut Ui) -> f32 {
    // TODO: make this configurable
    const MAX_TEXTEDIT_HEIGHT: f32 = 180.0;

    let layout = Layout::right_to_left(egui::Align::BOTTOM);

    // passing size out of the scroll area's size
    let desired_height = ui
        .with_layout(layout, |ui| {
            let send_button_clicked = ui.button("Send").clicked();
            let buttons_height = ui.min_size().y;

            let scroll_area = SizedVerticalScrollArea::new("TextInputScrollArea");

            let mut store_lock = state.ui_store.lock();
            let text = store_lock.message_text_input();

            let textbox_height = scroll_area
                .show(ui, |ui| {
                    TextEdit::multiline(text)
                        .frame(false)
                        .hint_text("Write a message...")
                        .desired_rows(1)
                        .desired_width(ui.available_width())
                        .show(ui);

                    let desired_height = ui.min_size().y.min(MAX_TEXTEDIT_HEIGHT);

                    // one of them is consumed by the scroll area, the other one we will use futher down
                    (desired_height, desired_height)
                })
                .inner;

            if send_button_clicked && !text.trim().is_empty() {
                let message = text.trim().to_owned();
                text.clear();

                if let Some(other_id) = store_lock.other_id() {
                    state.run_async(actions::send_message(
                        state.clone(),
                        message.trim().to_owned(),
                        store_lock.self_id(),
                        other_id,
                    ));
                }
            }

            let desired_height = f32::max(buttons_height, textbox_height);
            desired_height
        })
        .inner;
    desired_height
}
