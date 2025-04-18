use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use serde_json::{from_str, to_string};

use edsm_dumps_model::model::body::Body;
use edsm_dumps_model::model::powerplay::PowerPlay;
use edsm_dumps_model::model::station::Station;
use edsm_dumps_model::model::system::{SystemWithCoordinates, SystemWithoutCoordinates};
use edsm_dumps_model::model::system_populated::SystemPopulated;
use edsm_dumps_model::model::RootEntry;

#[test]
#[ignore]
fn parse_bodies_7days() -> Result<()> {
    try_parse::<Body>("./dumps/bodies7days.json.gz")
}

#[test]
#[ignore]
fn parse_power_play() -> Result<()> {
    try_parse::<PowerPlay>("./dumps/powerPlay.json.gz")
}

#[test]
#[ignore]
fn parse_stations() -> Result<()> {
    try_parse::<Station>("./dumps/stations.json.gz")
}

#[test]
#[ignore]
fn parse_systems_populated() -> Result<()> {
    try_parse::<SystemPopulated>("./dumps/systemsPopulated.json.gz")
}

#[test]
#[ignore]
fn parse_systems_with_coordinates_7days() -> Result<()> {
    try_parse::<SystemWithCoordinates>("./dumps/systemsWithCoordinates7days.json.gz")
}

#[test]
#[ignore]
fn parse_systems_without_coordinates() -> Result<()> {
    try_parse::<SystemWithoutCoordinates>("./dumps/systemsWithoutCoordinates.json.gz")
}

fn try_parse<T: RootEntry + std::fmt::Debug + PartialEq>(path: &str) -> Result<()> {
    let f = File::open(path).context("failed to read file")?;
    let r = GzDecoder::new(f);
    let r = BufReader::new(r);

    for (line_num, line) in r.lines().enumerate() {
        let line = line?;
        let line = line.trim().trim_end_matches(',');

        if line == "[" {
            continue;
        }
        if line == "]" {
            break;
        }

        try_round_trip::<T>(line).with_context(|| format!("failed at line {}", line_num + 1))?;
    }

    Ok(())
}

fn try_round_trip<T: RootEntry + std::fmt::Debug + PartialEq>(line: &str) -> Result<()> {
    let decoded = T::parse_dump_json(line.as_bytes())
        .with_context(|| format!("parsing sample JSON: {}", line))?;

    let encoded = to_string(&decoded).context("encoding decoded valuew to JSON")?;

    let re_decoded: T =
        from_str(&encoded).with_context(|| format!("parsing encoded value: {}", encoded))?;

    assert_eq!(
        decoded, re_decoded,
        "parsed value and re-parsed value should matches\nDecoded: {:?}\nRe-decoded: {:?}",
        decoded, re_decoded
    );

    Ok(())
}
