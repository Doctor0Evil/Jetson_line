use crate::microsoul::MicrosoulMetrics;
use crate::model::SiteIndex;

impl Episode {
    /// Compute MicrosoulMetrics for multiple branches that partition the line.
    pub fn compute_microsoul_metrics_for_branches(
        &self,
        branch_ranges: &[(SiteIndex, SiteIndex)],
    ) -> Vec<MicrosoulMetrics> {
        branch_ranges
            .iter()
            .map(|(start, end)| {
                MicrosoulMetrics::from_episode_branch(self, format!("branch_{}_{}", start, end), *start, *end)
            })
            .collect()
    }
}
