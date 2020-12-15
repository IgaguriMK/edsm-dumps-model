use std::io::{BufRead, BufReader};

use anyhow::Result;

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

fn try_parse<T: RootEntry>(bs: &[u8]) -> Result<()> {
    let r = BufReader::new(bs);

    for line in r.lines() {
        let line = line?;
        let line = line.trim().trim_end_matches(',');

        if line == "[" {
            continue;
        }
        if line == "]" {
            break;
        }

        let _ = T::parse_dump_json(line.as_bytes())?;
    }

    Ok(())
}
