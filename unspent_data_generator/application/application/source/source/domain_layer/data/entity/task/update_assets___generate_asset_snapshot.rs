use super::{
    generate_asset_snapshot::GenerateAssetSnapshot,
    update_assets::UpdateAssets,
    NamedTask,
};

pub type UpdateAssets__GenerateAssetSnapshot = (
    UpdateAssets,
    GenerateAssetSnapshot,
);

impl NamedTask for UpdateAssets__GenerateAssetSnapshot {
    fn get_name() -> &'static str {
        return "UpdateAssets__GenerateAssetSnapshot";
    }
}
