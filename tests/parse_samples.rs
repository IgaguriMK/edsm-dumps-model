//! Lightweight tests for parsing the sampled JSON files.
//!
//! The tests are not exhaustive, but they ensure that the sampled JSON files can be parsed and
//! serialized.
//! They also verify that the values remain identical after a round‑trip (parse → serialize → parse),
//! helping detect missing fields.

use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};
use serde_json::{from_str, to_string};

use edsm_dumps_model::model::body::Body;
use edsm_dumps_model::model::powerplay::PowerPlay;
use edsm_dumps_model::model::station::Station;
use edsm_dumps_model::model::system::{SystemWithCoordinates, SystemWithoutCoordinates};
use edsm_dumps_model::model::system_populated::SystemPopulated;
use edsm_dumps_model::model::RootEntry;

#[test]
fn parse_body() -> Result<()> {
    try_parse::<Body>("./sampled_json/body.json")
}

#[test]
fn parse_power_play() -> Result<()> {
    try_parse::<PowerPlay>("./sampled_json/powerPlay.json")
}

#[test]
fn parse_station() -> Result<()> {
    try_parse::<Station>("./sampled_json/station.json")
}

#[test]
fn parse_system_with_coordinates() -> Result<()> {
    try_parse::<SystemWithCoordinates>("./sampled_json/systemWithCoordinates.json")
}

#[test]
fn parse_system_without_coordinates() -> Result<()> {
    try_parse::<SystemWithoutCoordinates>("./sampled_json/systemWithoutCoordinates.json")
}

#[test]
fn parse_system_populated() -> Result<()> {
    try_parse::<SystemPopulated>("./sampled_json/systemPopulated.json")
}

fn try_parse<T: RootEntry + std::fmt::Debug + PartialEq>(path: &str) -> Result<()> {
    let f = File::open(path).context("failed to read file")?;
    let r = BufReader::new(f);

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
