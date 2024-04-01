use client::{gui::Gui, state::AppState, gui::store::Store};
use eframe::NativeOptions;
use eyre::eyre;

fn main() -> eyre::Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    let store = Store::new();
    let app_state_factory = AppState::factory(store, rt.handle().clone());
    let native_options = NativeOptions::default();

    eframe::run_native("SoChat", native_options, Gui::app_creator(app_state_factory))
        .map_err(|e| eyre!("{e}"))?;

    Ok(())
}
