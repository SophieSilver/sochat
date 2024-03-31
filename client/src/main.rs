use client::{gui::Gui, state::AppState};
use eframe::NativeOptions;
use eyre::eyre;

fn main() -> eyre::Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    let state_factory = AppState::factory(rt.handle().clone());
    let native_options = NativeOptions::default();

    eframe::run_native("SoChat", native_options, Gui::app_creator(state_factory))
        .map_err(|e| eyre!("{e}"))?;

    Ok(())
}
