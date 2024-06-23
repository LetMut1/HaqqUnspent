use super::Resolver;
pub use crate::infrastructure_layer::data::control_type::SnapshotRange;

impl Resolver<SnapshotRange> {
    pub const REGISTRY: [&'static str; 5] = [
        "1d",
        "7d",
        "1m",
        "1y",
        "all",
    ];
}
