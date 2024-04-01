use crate::state::AppState;
use eframe::{AppCreator, CreationContext};

mod components {
    pub mod message_display_panel;
    pub mod text_input_panel;
}
use components::{
    message_display_panel::show_message_display_panel, text_input_panel::TextInputPanelState,
};

pub mod store;

pub struct Gui {
    state: AppState,
    text_input_panel: TextInputPanelState,
}

impl Gui {
    pub fn new<F>(cc: &CreationContext, state_factory: F) -> Self
    where
        F: FnOnce(&CreationContext) -> AppState,
    {
        Self {
            state: state_factory(cc),
            text_input_panel: TextInputPanelState::new(),
        }
    }

    pub fn app_creator<F>(state_factory: F) -> AppCreator
    where
        F: FnOnce(&CreationContext) -> AppState + 'static,
    {
        Box::new(move |cc| Box::new(Self::new(cc, state_factory)))
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // TODO: make a way to set up styling
        self.text_input_panel.show(&self.state, ctx);

        show_message_display_panel(&self.state.ui_store, ctx);
    }
}
