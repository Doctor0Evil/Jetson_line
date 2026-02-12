impl Episode {
    /// Append a single deed / rule-trigger event to the log.
    pub fn log_event(
        &mut self,
        tick: Tick,
        agent_id: AgentId,
        group_id: Option<GroupId>,
        deed_type: impl Into<String>,
        rule_id: RuleId,
        pre_tokens: TokenState,
        post_tokens: TokenState,
        local_social_impact: f64,
        local_biophysical_impact: f64,
    ) {
        let entry = EventLogEntry {
            tick,
            agent_id,
            group_id,
            deed_type: deed_type.into(),
            rule_id,
            pre_tokens,
            post_tokens,
            local_social_impact,
            local_biophysical_impact,
        };
        self.event_log.push(entry);
    }
}
