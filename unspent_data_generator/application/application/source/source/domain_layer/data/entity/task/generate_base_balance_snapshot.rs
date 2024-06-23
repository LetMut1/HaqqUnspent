use super::NamedTask;

pub struct GenerateBaseBalanceSnapshot;

impl NamedTask for GenerateBaseBalanceSnapshot {
    fn get_name() -> &'static str {
        return "GenerateBaseBalanceSnapshot";
    }
}
