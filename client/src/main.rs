use client::{
    api,
    gui::{store::Store, Gui},
    state::AppState,
};
use eframe::NativeOptions;
use eyre::eyre;

fn main() -> eyre::Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    let user_id = rt.block_on(api::register_user())?;

    println!("{:?}", user_id);

    let store = Store::new(user_id);
    let app_state_factory = AppState::factory(store, rt.handle().clone());
    let native_options = NativeOptions::default();

    eframe::run_native(
        "SoChat",
        native_options,
        Gui::app_creator(app_state_factory),
    )
    .map_err(|e| eyre!("{e}"))?;

    Ok(())
}
