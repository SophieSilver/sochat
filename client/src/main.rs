use std::mem;

use egui::{
    Color32, Frame, Key, KeyboardShortcut, Layout, Margin, Modifiers, RichText, Rounding,
    ScrollArea, TextEdit,
};

#[derive(Debug, Clone, Default)]
struct MyApp {
    messages: Vec<String>,
    text: String,
    size: f32,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            messages: Vec::new(),
            text: String::new(),
            size: 0.0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("Typing Area")
            .resizable(false)
            .min_height(0.0)
            .max_height(self.size.min(200.0))
            .show_separator_line(false)
            .frame(
                Frame::central_panel(&ctx.style())
                    .inner_margin(0.0)
                    .outer_margin(0.0), //.stroke(Stroke::new(1.0, Color32::DARK_BLUE)),
            )
            .show(ctx, |ui| {
                let frame = Frame::default()
                    .inner_margin(10.0)
                    .fill(Color32::from_gray(36));
                let total_margin = frame.total_margin();
                let inner_height = frame
                    .show(ui, |ui| {
                        let layout = Layout::right_to_left(egui::Align::BOTTOM);
                        ui.allocate_ui_with_layout(ui.available_size(), layout, |ui| {
                            let clicked_send = ui.button("Send").clicked();
                            let button_height = ui.min_size().y;

                            let text_scroll = ScrollArea::vertical()
                                .min_scrolled_height((self.size - 20.0).min(200.0 - 20.0))
                                .stick_to_bottom(true);

                            let text_edit = TextEdit::multiline(&mut self.text)
                                .frame(false)
                                .hint_text("Write a message...")
                                .desired_rows(1)
                                .desired_width(ui.available_width())
                                .return_key(KeyboardShortcut::new(Modifiers::SHIFT, Key::Enter))
                                .id("TextArea".into());

                            let response = text_scroll.show(ui, |ui| text_edit.show(ui).response);

                            let has_focus = response.inner.has_focus();

                            if !has_focus {
                                ui.ctx().memory_mut(|m| m.request_focus("TextArea".into()));
                            }

                            // lazy evaluation
                            let pressed_enter = || {
                                ui.ctx().input(|i| {
                                    i.key_released(egui::Key::Enter) && !i.modifiers.shift
                                })
                            };

                            if clicked_send
                                || (has_focus && pressed_enter()) && !self.text.is_empty()
                            {
                                self.messages
                                    .push(mem::replace(&mut self.text, String::new()));
                            };
                            response.content_size.y.max(button_height)
                        })
                        .inner
                    })
                    .inner;
                let outer_height = inner_height + total_margin.bottom + total_margin.top;
                self.size = outer_height;
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            let scroll = ScrollArea::vertical().stick_to_bottom(true);
            scroll.show(ui, |ui| {
                ui.allocate_space(egui::vec2(ui.available_width(), 0.0));
                for message in &self.messages {
                    let frame = Frame::central_panel(&ctx.style())
                        .fill(Color32::from_gray(70))
                        .rounding(Rounding {
                            nw: 20.0,
                            ne: 20.0,
                            sw: 5.0,
                            se: 20.0,
                        })
                        .inner_margin(Margin::symmetric(16.0, 8.0));

                    frame.show(ui, |ui| {
                        //    ui.allocate_space(egui::vec2(30.0, 0.0));
                        ui.label(RichText::new(message).color(Color32::WHITE));
                    });
                }
            });
        });
    }
}

fn main() -> eyre::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My App",
        native_options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
    .map_err(|e| eyre::anyhow!("{e}"))?;

    Ok(())
}
