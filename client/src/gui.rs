use crate::state::AppState;
use eframe::{AppCreator, CreationContext};

use self::text_input_panel::TextInputPanel;

mod text_input_panel;

pub struct Gui {
    state: AppState,
    text_input_panel: TextInputPanel,
}

impl Gui {
    pub fn new(cc: &CreationContext, state: AppState) -> Self {
        Self {
            state,
            text_input_panel: TextInputPanel::new(),
        }
    }

    pub fn app_creator(state: AppState) -> AppCreator {
        Box::new(move |cc| Box::new(Self::new(cc, state)))
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // TODO: make a way to set up styling
        self.text_input_panel.show(&self.state, ctx);
        //        CentralPanel::default().show(ctx, |ui| {});
    }
}