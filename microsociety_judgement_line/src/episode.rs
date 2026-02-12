use std::collections::{HashMap, HashSet};

impl Episode {
    pub fn compute_metrics(
        &self,
    ) -> (Vec<AgentMetrics>, Vec<GroupMetrics>, SystemMetrics) {
        let agent_metrics = self.compute_agent_metrics();
        let group_metrics = self.compute_group_metrics();
        let system_metrics = self.compute_system_metrics(&agent_metrics, &group_metrics);
        (agent_metrics, group_metrics, system_metrics)
    }

    fn compute_agent_metrics(&self) -> Vec<AgentMetrics> {
        let mut per_agent_events: HashMap<AgentId, Vec<&EventLogEntry>> = HashMap::new();
        let mut active_ticks: HashMap<AgentId, HashSet<Tick>> = HashMap::new();

        for e in &self.event_log {
            per_agent_events.entry(e.agent_id).or_default().push(e);
            active_ticks.entry(e.agent_id).or_default().insert(e.tick);
        }

        let mut metrics = Vec::new();
        let max_tick = self.event_log.last().map(|e| e.tick).unwrap_or(0);
        let total_ticks = (max_tick + 1) as f64;

        for (agent_id, events) in per_agent_events {
            let ticks_active = active_ticks
                .get(&agent_id)
                .map(|s| s.len() as f64)
                .unwrap_or(0.0);

            let survival_fraction = if total_ticks > 0.0 {
                ticks_active / total_ticks
            } else {
                0.0
            };

            // Average tokens from post_states.
            let mut sum = TokenState { church: 0.0, fear: 0.0, power: 0.0, tech: 0.0 };
            let mut count = 0.0;
            let mut coop_count = 0.0;
            let mut aggressive_count = 0.0;
            let mut restorative_deeds = 0u32;
            let mut justice_accum = 0.0;

            for e in events {
                sum.church += e.post_tokens.church;
                sum.fear += e.post_tokens.fear;
                sum.power += e.post_tokens.power;
                sum.tech += e.post_tokens.tech;
                count += 1.0;

                // Example tagging: treat deed types starting with "coop_" vs "agg_".
                if e.deed_type.starts_with("coop_") {
                    coop_count += 1.0;
                } else if e.deed_type.starts_with("agg_") {
                    aggressive_count += 1.0;
                }

                if e.deed_type.starts_with("restorative_") {
                    restorative_deeds += 1;
                }

                // Simple justice proxy: higher church interpreted as higher perceived justice.
                justice_accum += e.post_tokens.church;
            }

            let avg_tokens = if count > 0.0 {
                TokenState {
                    church: sum.church / count,
                    fear: sum.fear / count,
                    power: sum.power / count,
                    tech: sum.tech / count,
                }
            } else {
                sum
            };

            let denom = coop_count + aggressive_count;
            let coop_ratio = if denom > 0.0 { coop_count / denom } else { 0.0 };

            let perceived_justice = if count > 0.0 {
                justice_accum / count
            } else {
                0.0
            };

            metrics.push(AgentMetrics {
                agent_id,
                survival_fraction,
                avg_tokens,
                coop_ratio,
                perceived_justice,
                restorative_deeds,
            });
        }

        metrics
    }

    fn compute_group_metrics(&self) -> Vec<GroupMetrics> {
        let mut per_group_events: HashMap<GroupId, Vec<&EventLogEntry>> = HashMap::new();

        for e in &self.event_log {
            if let Some(gid) = e.group_id {
                per_group_events.entry(gid).or_default().push(e);
            }
        }

        let mut metrics = Vec::new();

        for (group_id, events) in per_group_events {
            let mut conflict_events = 0u32;
            let mut reconciliation_events = 0u32;

            let mut powers = Vec::new();
            let mut churches = Vec::new();
            let mut techs = Vec::new();

            for e in events {
                if e.deed_type.starts_with("agg_") {
                    conflict_events += 1;
                } else if e.deed_type.starts_with("reconcile_") {
                    reconciliation_events += 1;
                }

                powers.push(e.post_tokens.power);
                churches.push(e.post_tokens.church);
                techs.push(e.post_tokens.tech);
            }

            let gini_power = gini(&powers);
            let gini_church = gini(&churches);
            let gini_tech = gini(&techs);

            // Placeholder: alliance stability to be wired to your alliance tracking.
            let alliance_stability = 0.0;

            metrics.push(GroupMetrics {
                group_id,
                conflict_events,
                reconciliation_events,
                gini_power,
                gini_church,
                gini_tech,
                alliance_stability,
            });
        }

        metrics
    }

    fn compute_system_metrics(
        &self,
        agent_metrics: &[AgentMetrics],
        group_metrics: &[GroupMetrics],
    ) -> SystemMetrics {
        let tick_end = self.event_log.last().map(|e| e.tick).unwrap_or(0);

        // System token totals as sum of last seen tokens per agent.
        let mut total = TokenState { church: 0.0, fear: 0.0, power: 0.0, tech: 0.0 };
        for m in agent_metrics {
            total.church += m.avg_tokens.church;
            total.fear += m.avg_tokens.fear;
            total.power += m.avg_tokens.power;
            total.tech += m.avg_tokens.tech;
        }

        let mut coop_deeds = 0.0;
        let mut agg_deeds = 0.0;
        let mut conflict_events = 0.0;

        for e in &self.event_log {
            if e.deed_type.starts_with("coop_") {
                coop_deeds += 1.0;
            } else if e.deed_type.starts_with("agg_") {
                agg_deeds += 1.0;
                conflict_events += 1.0;
            }
        }

        let total_deeds = coop_deeds + agg_deeds;
        let cooperation_index = if total_deeds > 0.0 {
            coop_deeds / total_deeds
        } else {
            0.0
        };

        let conflict_intensity = if tick_end > 0 {
            conflict_events / (tick_end as f64 + 1.0)
        } else {
            0.0
        };

        // Simple resilience placeholders (wire to your shock detection).
        let shock_count = 0;
        let avg_recovery_time = 0.0;

        // System inequality from group-level tokens (power, church, tech merged).
        let mut power_vals = Vec::new();
        let mut church_vals = Vec::new();
        let mut tech_vals = Vec::new();

        for gm in group_metrics {
            power_vals.push(gm.gini_power);   // you might use raw group means instead
            church_vals.push(gm.gini_church);
            tech_vals.push(gm.gini_tech);
        }

        let system_gini_power = gini(&power_vals);
        let system_gini_church = gini(&church_vals);
        let system_gini_tech = gini(&tech_vals);

        SystemMetrics {
            tick_end,
            occupied_fraction: 0.0, // fill from your lattice if available
            total_tokens: total,
            cooperation_index,
            conflict_intensity,
            shock_count,
            avg_recovery_time,
            system_gini_power,
            system_gini_church,
            system_gini_tech,
        }
    }
}

/// Simple Gini coefficient over non-negative values.
fn gini(values: &[f64]) -> f64 {
    let n = values.len();
    if n == 0 {
        return 0.0;
    }
    let mut vals: Vec<f64> = values.iter().cloned().filter(|v| *v >= 0.0).collect();
    vals.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = vals.len();
    if n == 0 {
        return 0.0;
    }
    let sum: f64 = vals.iter().sum();
    if sum == 0.0 {
        return 0.0;
    }
    let mut cum = 0.0;
    for (i, v) in vals.iter().enumerate() {
        cum += (i as f64 + 1.0) * *v;
    }
    (2.0 * cum) / (n as f64 * sum) - (n as f64 + 1.0) / (n as f64)
}
