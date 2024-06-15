pub mod id;

pub mod marker {
    use flutter_rust_bridge::frb;

    #[frb(opaque)]
    pub struct Opaque {}
}
