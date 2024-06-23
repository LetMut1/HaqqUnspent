use super::NamedTask;

pub struct GenerateAggregatedBalanceSnapshot;

impl NamedTask for GenerateAggregatedBalanceSnapshot {
    fn get_name() -> &'static str {
        return "GenerateAggregatedBalanceSnapshot";
    }
}
