use super::NamedTask;

pub struct GenerateAssetSnapshot;

impl NamedTask for GenerateAssetSnapshot {
    fn get_name() -> &'static str {
        return "GenerateAssetSnapshot";
    }
}
