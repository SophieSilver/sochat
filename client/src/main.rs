use client::{gui::Gui, state::AppState};
use eframe::NativeOptions;
use eyre::eyre;

fn main() -> eyre::Result<()> {
    let state = AppState::new();
    let native_options = NativeOptions::default();

    eframe::run_native("SoChat", native_options, Gui::app_creator(state))
        .map_err(|e| eyre!("{e}"))?;

    Ok(())
}
