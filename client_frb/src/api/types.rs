pub mod id;

pub mod marker {
    use flutter_rust_bridge::frb;

    #[frb(opaque)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Opaque {}
}
