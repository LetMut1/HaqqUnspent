use super::NamedTask;

pub struct UpdateAssets;

impl NamedTask for UpdateAssets {
    fn get_name() -> &'static str {
        return "UpdateAssets";
    }
}
