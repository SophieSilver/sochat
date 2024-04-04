use egui::{Color32, Context, Frame, Margin, RichText, Rounding, ScrollArea, Ui};

use crate::gui::store::Store;

pub fn show(state: &Store, ctx: &Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let scroll_area = ScrollArea::vertical().stick_to_bottom(true);
        scroll_area.show(ui, |ui| {
            // this is so that the scrollbar appears all the way at the left and not somewhere in the middle
            ui.allocate_space(egui::vec2(ui.available_width(), 0.0));
            show_messages(state, ui);
        });
    });
}

fn show_messages(state: &Store, ui: &mut Ui) {
    let state = state.lock_blocking();
    let messages = state.messages();

    for message in messages {
        let frame = Frame::central_panel(ui.style())
            .fill(Color32::from_gray(70))
            .rounding(Rounding {
                nw: 20.0,
                ne: 20.0,
                sw: 5.0,
                se: 20.0,
            })
            .inner_margin(Margin::symmetric(12.0, 8.0));
        
        frame.show(ui, |ui| {
            ui.label(RichText::new(message).color(Color32::WHITE));
        });
    }
}
