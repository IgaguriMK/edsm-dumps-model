use std::env::var;
use std::path::Path;

use anyhow::{Context, Result};

use edsm_dumps_model::array_decoder::parallel::ParallelDecoder;
use edsm_dumps_model::array_decoder::NopProgress;
use edsm_dumps_model::model::body::Body;
use edsm_dumps_model::model::powerplay::PowerPlay;
use edsm_dumps_model::model::station::Station;
use edsm_dumps_model::model::system::{SystemWithCoordinates, SystemWithoutCoordinates};
use edsm_dumps_model::model::system_populated::SystemPopulated;
use edsm_dumps_model::model::RootEntry;

#[test]
#[ignore]
fn parse_test() -> Result<()> {
    let dumps_dir = var("EDSM_DUMPS_DIR").context("reading dumps dir path from env")?;
    let dumps_dir = Path::new(&dumps_dir);

    check::<PowerPlay>(&dumps_dir.join("powerPlay.json"))?;
    check::<Station>(&dumps_dir.join("stations.json"))?;
    check::<SystemPopulated>(&dumps_dir.join("systemsPopulated.json"))?;

    let p = dumps_dir.join("bodies.json");
    if p.exists() {
        check::<Body>(&p)?;
    } else {
        check::<Body>(&dumps_dir.join("bodies7days.json"))?;
    }

    check::<SystemWithCoordinates>(&dumps_dir.join("systemsWithCoordinates.json"))?;
    check::<SystemWithoutCoordinates>(&dumps_dir.join("systemsWithoutCoordinates.json"))?;

    Ok(())
}

fn check<T: RootEntry>(path: &Path) -> Result<()> {
    let mut dec = ParallelDecoder::<T>::start(path, NopProgress)
        .with_context(|| format!("opening decoder for {}: {:?}", T::type_name(), path))?;

    let mut entry_count = 0usize;
    loop {
        match dec.read_entry() {
            Ok(Some(_)) => {
                entry_count += 1;
            }
            Ok(None) => break,
            Err(e) => {
                panic!("Parse failed for {}: {:#}", T::type_name(), e);
            }
        }
    }
    assert!(
        entry_count > 0,
        "No entry for {}: {:?}",
        T::type_name(),
        path
    );

    Ok(())
}
