use egui::{
    Align, Color32, Context, Frame, Layout, Margin, RichText, Rounding, ScrollArea, Style,
    Ui,
};

use crate::gui::store::Store;

pub fn show(store: &Store, ctx: &Context) {
    egui::CentralPanel::default()
        .frame(Frame::central_panel(&ctx.style()).fill(Color32::from_gray(18)))
        .show(ctx, |ui| {
            let scroll_area = ScrollArea::vertical().stick_to_bottom(true);
            scroll_area.show(ui, |ui| {
                // this is so that the scrollbar appears all the way at the left and not somewhere in the middle
                ui.allocate_space(egui::vec2(ui.available_width(), 0.0));
                show_messages(store, ui);
            });
        });
}

fn show_messages(store: &Store, ui: &mut Ui) {
    let store = store.lock();
    let messages = store.messages();

    for message in messages {
        let (frame, layout) = if message.sender_id == store.self_id() {
            self_message_box_frame_and_layout(ui.style())
        } else {
            other_message_box_frame_and_layout(ui.style())
        };

        ui.with_layout(layout, |ui| {
            frame.show(ui, |ui| {
                ui.set_max_width((ui.available_width()).min(400.0));
                ui.label(RichText::new(&message.content).color(Color32::WHITE));
            });
        });
    }
}

fn self_message_box_frame_and_layout(style: &Style) -> (Frame, Layout) {
    let frame = Frame::central_panel(style)
        .fill(Color32::from_gray(84))
        .rounding(Rounding {
            nw: 15.0,
            ne: 15.0,
            sw: 15.0,
            se: 5.0,
        })
        .inner_margin(Margin::symmetric(12.0, 8.0))
        .outer_margin(Margin {
            left: 15.0,
            right: 5.0,
            top: 0.0,
            bottom: 0.0,
        });

    let layout = Layout::right_to_left(Align::TOP).with_main_wrap(true);

    (frame, layout)
}

fn other_message_box_frame_and_layout(style: &Style) -> (Frame, Layout) {
    let frame = Frame::central_panel(style)
        .fill(Color32::from_gray(38))
        .rounding(Rounding {
            nw: 15.0,
            ne: 15.0,
            sw: 5.0,
            se: 15.0,
        })
        .inner_margin(Margin::symmetric(12.0, 8.0))
        .outer_margin(Margin {
            left: 5.0,
            right: 15.0,
            top: 0.0,
            bottom: 0.0,
        });

    let layout = Layout::left_to_right(Align::TOP).with_main_wrap(true);

    (frame, layout)
}
