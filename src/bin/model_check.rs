use std::path::Path;
use std::fs::File;
use std::io::BufReader;

use tiny_fail::{ErrorMessageExt, Fail};
use serde::de::DeserializeOwned;

use edsm_dumps_downloader::config::Config;
use edsm_dumps_downloader::array_decoder::ArrayDecoder;
use edsm_dumps_downloader::model::{SystemWithCoordinates};

fn main() {
    if let Err(fail) = w_main() {
        eprintln!("Error: {}", fail);
        std::process::exit(1);
    }
}

fn w_main() -> Result<(), Fail> {
    let cfg = Config::load("./config.toml").err_msg("failed load config file")?;

    check_parse::<SystemWithCoordinates>(cfg.dumps_dir().as_ref(), "systemsWithCoordinates.json")?;

    Ok(())
}

fn check_parse<D: DeserializeOwned>(dir: &Path, file_name: &str) -> Result<(), Fail> {
    let path = dir.join(file_name);
    let f = File::open(&path).err_msg(format!("failed open dump file '{:?}'", path))?;
    let r = BufReader::new(f);
    let mut dec = ArrayDecoder::new(r);

    while let Some(_) = dec.next::<D>().err_msg(format!("While checking '{}'", file_name))? {}

    Ok(())
}