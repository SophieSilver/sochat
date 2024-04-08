use common::types::UserId;
use egui::{Color32, Context, FontSelection, RichText, SidePanel, TextEdit, Ui};

use crate::{actions, state::AppState};

// TODO: rewrite pretty much all if this
pub fn show(state: &AppState, ctx: &Context) {
    SidePanel::left("SidePanel")
        .resizable(false)
        .show(ctx, |ui| {
            let mut store_lock = state.ui_store.lock();

            let self_id_string = store_lock.self_id().to_string();
            let id_string_length = self_id_string.len();

            ui.label("Your ID:");
            show_copyable_id(ui, self_id_string);

            ui.add_space(10.0);

            match store_lock.other_id() {
                Some(other_id) => {
                    ui.label("Other person's ID:");
                    show_copyable_id(ui, other_id.to_string());
                }
                None => {
                    ui.label("Enter other person's ID:");
                    let text_edit = TextEdit::singleline(store_lock.other_id_input_line())
                        .font(FontSelection::Style(egui::TextStyle::Monospace))
                        .char_limit(id_string_length);
                    ui.add(text_edit);

                    if !ui.button("Confirm").clicked() {
                        return;
                    }

                    let Ok(other_id) = store_lock.other_id_input_line().parse::<UserId>() else {
                        return;
                    };
                    store_lock.set_other_id(other_id);
                    state.run_async(actions::start_conversation(state.clone(), other_id));
                }
            }
        });
}

fn show_copyable_id(ui: &mut Ui, id_string: String) {
    let id_label = egui::Label::new(
        RichText::new(id_string.clone())
            .monospace()
            .color(Color32::from_gray(200)),
    )
    .wrap(false);
    ui.add(id_label);

    if ui.button("Copy").clicked() {
        ui.ctx().copy_text(id_string);
    }
}
