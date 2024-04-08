use crate::state::AppState;
use eframe::{AppCreator, CreationContext};

mod components {
    pub mod message_display_panel;
    pub mod text_input_panel;
    pub mod side_panel;
}
pub mod store;

use self::components::{message_display_panel, side_panel, text_input_panel};


pub struct Gui {
    state: AppState,
}

impl Gui {
    pub fn new<F>(cc: &CreationContext, state_factory: F) -> Self
    where
        F: FnOnce(&CreationContext) -> AppState,
    {
        cc.egui_ctx.set_pixels_per_point(1.5);
        
        Self {
            state: state_factory(cc),
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
        side_panel::show(&self.state, ctx);
        text_input_panel::show(&self.state, ctx);

        message_display_panel::show(&self.state.ui_store, ctx);
    }
}
