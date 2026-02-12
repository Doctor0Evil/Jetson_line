use chrono::Utc;
use microsociety_judgement_line::{
    BiophysicalEvidence, CitizenId, Deed, DeedKind, JetsonLine, ScoringParameters,
};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// Input format for biophysical events captured on Jetson.
/// This can be emitted by your sensor/AI pipelines.
#[derive(Debug, serde::Deserialize)]
struct RawEvent {
    citizen_id: String,
    kind: String,
    description: String,
    evidence_source: String,
    evidence_reference: String,
    occurred_at: String,
    recorded_at: String,
}

/// Load a batch of raw events from a JSON file produced on-device.
fn load_raw_events<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<RawEvent>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let events: Vec<RawEvent> = serde_json::from_str(&buf)?;
    Ok(events)
}

/// Map string kind into DeedKind, constrained to non-fictional categories.
fn parse_deed_kind(kind: &str) -> DeedKind {
    match kind {
        "good_deed" => DeedKind::GoodDeed,
        "heroic_action" => DeedKind::HeroicAction,
        "harmful_action" => DeedKind::HarmfulAction,
        _ => DeedKind::NeutralAction,
    }
}

/// Convert raw events into a JetsonLine instance.
fn build_line_from_events(events: Vec<RawEvent>) -> anyhow::Result<JetsonLine> {
    let mut line = JetsonLine::new();

    for (idx, ev) in events.into_iter().enumerate() {
        let occurred_at = ev.occurred_at.parse::<chrono::DateTime<Utc>>()?;
        let recorded_at = ev.recorded_at.parse::<chrono::DateTime<Utc>>()?;

        let deed = Deed {
            id: idx as u64,
            citizen: CitizenId(ev.citizen_id),
            kind: parse_deed_kind(&ev.kind),
            description: ev.description,
            evidence: vec![BiophysicalEvidence {
                source: ev.evidence_source,
                reference: ev.evidence_reference,
                recorded_at,
            }],
            occurred_at,
        };

        line.append_deed(deed)?;
    }

    Ok(line)
}

fn main() -> anyhow::Result<()> {
    // 1. Ingest biophysical events from a file produced by Jetson pipelines.
    let events = load_raw_events("data/biophysical_events.json")?;

    // 2. Build the 1-D Jetson_Line structure.
    let line = build_line_from_events(events)?;

    // 3. Compute scores per citizen (for CHURCH / POWER eligibility upstream).
    let params = ScoringParameters::default();
    let now = Utc::now();

    // Simple aggregation: collect scores by citizen ID.
    use std::collections::HashMap;
    let mut scores: HashMap<String, f64> = HashMap::new();

    for deed in line.deeds() {
        let citizen_id = deed.citizen.0.clone();
        let entry = scores.entry(citizen_id).or_insert(0.0);
        let single_score = microsociety_judgement_line::compute_moral_score(
            &[deed.clone()],
            &params,
            now,
        );
        *entry += single_score.0;
    }

    // 4. Export line + scores to JSON for Tree-of-Life / Church-of-FEAR.
    let line_json = line.to_json()?;
    std::fs::create_dir_all("out")?;
    std::fs::write("out/jetson_line.json", line_json)?;

    let scores_json = serde_json::to_string_pretty(&scores)?;
    std::fs::write("out/citizen_scores.json", scores_json)?;

    Ok(())
}
