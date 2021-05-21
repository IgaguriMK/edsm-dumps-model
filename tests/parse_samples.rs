use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};
use serde_json::{from_slice, to_vec};

use edsm_dumps_model::model::body::Body;
use edsm_dumps_model::model::powerplay::PowerPlay;
use edsm_dumps_model::model::station::Station;
use edsm_dumps_model::model::system::{SystemWithCoordinates, SystemWithoutCoordinates};
use edsm_dumps_model::model::system_populated::SystemPopulated;
use edsm_dumps_model::model::RootEntry;

#[test]
fn parse_body() -> Result<()> {
    let bs = include_bytes!("./sample_json/body.json");
    try_parse::<Body>(&bs[..])
}

#[test]
fn parse_power_play() -> Result<()> {
    let bs = include_bytes!("./sample_json/powerPlay.json");
    try_parse::<PowerPlay>(&bs[..])
}

#[test]
fn parse_station() -> Result<()> {
    let bs = include_bytes!("./sample_json/station.json");
    try_parse::<Station>(&bs[..])
}

#[test]
fn parse_system_with_coordinates() -> Result<()> {
    let bs = include_bytes!("./sample_json/systemWithCoordinates.json");
    try_parse::<SystemWithCoordinates>(&bs[..])
}

#[test]
fn parse_system_without_coordinates() -> Result<()> {
    let bs = include_bytes!("./sample_json/systemWithoutCoordinates.json");
    try_parse::<SystemWithoutCoordinates>(&bs[..])
}

#[test]
fn parse_system_populated() -> Result<()> {
    let bs = include_bytes!("./sample_json/systemPopulated.json");
    try_parse::<SystemPopulated>(&bs[..])
}

fn try_parse<T: RootEntry + std::fmt::Debug + PartialEq>(bs: &[u8]) -> Result<()> {
    let r = BufReader::new(bs);

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
    let decoded = T::parse_dump_json(line.as_bytes()).context("parsing sample JSON")?;

    let encoded = to_vec(&decoded).context("encoding decoded valuew to JSON")?;

    let re_decoded: T = from_slice(&encoded).context("parsing encoded value")?;

    assert_eq!(
        decoded, re_decoded,
        "parsed value and re-parsed value should matches"
    );

    Ok(())
}
