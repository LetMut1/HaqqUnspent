use super::NamedTask;

pub struct UpdateAssetsForSubportfolioTrackableWallet;

impl NamedTask for UpdateAssetsForSubportfolioTrackableWallet {
    fn get_name() -> &'static str {
        return "UpdateAssetsForSubportfolioTrackableWallet";
    }
}
