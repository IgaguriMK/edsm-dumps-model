use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::thread::spawn;

use anyhow::{Context, Error};
use clap::{App, Arg};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde_json::Value;

use edsm_dumps_model::array_decoder::{ArrayDecoder, Progress};
use edsm_dumps_model::config::Config;
use edsm_dumps_model::schema::criteria::{Criteria, Criterias};
use edsm_dumps_model::schema::SchemaGenerator;

fn main() {
    if let Err(err) = w_main() {
        eprintln!("Error: {}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| eprintln!("    because: {}", cause));
        std::process::exit(1);
    }
}

fn w_main() -> Result<(), Error> {
    let matches = App::new("gen_schema")
        .arg(
            Arg::with_name("target")
                .short("t")
                .long("target")
                .takes_value(true)
                .help("Specify check target"),
        )
        .get_matches();

    let cfg = Config::load("./config.toml").context("failed load config file")?;
    let criterias = Criterias::load("./criterias.json").context("failed load criterias file")?;

    let dumps_dir = cfg.dumps_dir();
    let mut generator = Generator::new(dumps_dir.as_ref(), matches.value_of("target"), criterias);

    generator.generate("bodies.json")?;
    generator.generate("powerPlay.json")?;
    generator.generate("stations.json")?;
    generator.generate("systemsPopulated.json")?;
    generator.generate("systemsWithCoordinates.json")?;
    generator.generate("systemsWithoutCoordinates.json")?;

    generator.join()?;

    Ok(())
}

struct Generator<'a> {
    dir: &'a Path,
    check_target: Option<&'a str>,
    criterias: Criterias,
    progresses: MultiProgress,
}

impl<'a> Generator<'a> {
    fn new(dir: &'a Path, check_target: Option<&'a str>, criterias: Criterias) -> Generator<'a> {
        Generator {
            dir,
            check_target,
            criterias,
            progresses: MultiProgress::new(),
        }
    }

    fn generate(&mut self, file_name: &str) -> Result<(), Error> {
        if let Some(check_target) = self.check_target {
            if check_target != file_name {
                return Ok(());
            }
        }

        let path = self.dir.join(&file_name);
        let size = path.metadata()?.len();
        let file_name = file_name.to_owned();

        let progress = CheckProgress(
            self.progresses
                .add(CheckProgress::new_bar(&file_name, size)),
        );

        let criteria = self.criterias.get(file_name.trim_end_matches(".json"));

        spawn(move || {
            if let Err(e) = gen(path, progress, file_name, criteria) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        });

        Ok(())
    }

    fn join(&mut self) -> Result<(), Error> {
        self.progresses.join()?;
        Ok(())
    }
}

fn gen(
    path: PathBuf,
    progress: CheckProgress,
    file_name: String,
    criteria: Criteria,
) -> Result<(), Error> {
    let mut dec = ArrayDecoder::open(&path, progress).context("")?;

    let mut schema_generator = SchemaGenerator::new(criteria);

    while let Some(val) = dec
        .read_entry::<Value>()
        .context(format!("While checking '{}'", file_name))?
    {
        schema_generator.add_value(val);
    }

    let schema = schema_generator.build();

    let mut w = BufWriter::new(File::create(format!(
        "schemas/{}.txt",
        file_name.trim_end_matches(".json")
    ))?);
    schema.print(&mut w)?;
    w.flush()?;

    Ok(())
}

struct CheckProgress(ProgressBar);

impl CheckProgress {
    fn new_bar(file_name: &str, size: u64) -> ProgressBar {
        let prog_bar = ProgressBar::new(size);
        prog_bar.set_style(ProgressStyle::default_bar().template(
            "{msg:25} {bar:32.green/white} {bytes:8}/{total_bytes:8}, {bytes_per_sec:9}, Time:{elapsed_precise} ETA:{eta_precise}",
        ));
        prog_bar.set_draw_delta(1024);
        prog_bar.set_message(file_name.trim_end_matches(".json"));
        prog_bar
    }
}

impl Progress for CheckProgress {
    fn inc(&mut self, delta: usize) {
        self.0.inc(delta as u64);
    }
}

impl Drop for CheckProgress {
    fn drop(&mut self) {
        self.0.finish();
    }
}
