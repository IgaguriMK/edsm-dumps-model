use std::path::{Path, PathBuf};
use std::thread::spawn;

use anyhow::{Context, Error};
use clap::{App, Arg};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use edsm_dumps_model::array_decoder::parallel::ParallelDecoder;
use edsm_dumps_model::array_decoder::Progress;
use edsm_dumps_model::model::body::Body;
use edsm_dumps_model::model::powerplay::PowerPlay;
use edsm_dumps_model::model::station::Station;
use edsm_dumps_model::model::system::{SystemWithCoordinates, SystemWithoutCoordinates};
use edsm_dumps_model::model::system_populated::SystemPopulated;
use edsm_dumps_model::model::RootEntry;

use edsm_dumps_model::config::Config;

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
    let matches = App::new("model_checker")
        .arg(
            Arg::with_name("target")
                .short("t")
                .long("target")
                .takes_value(true)
                .help("Specify check target"),
        )
        .arg(
            Arg::with_name("seq-file")
                .short("F")
                .long("seq-file")
                .help("Check files sequentially."),
        )
        .get_matches();

    let cfg = Config::load("./config.toml").context("failed load config file")?;

    let dumps_dir = cfg.dumps_dir();
    let mut checker = Checker::new(dumps_dir.as_ref(), matches.value_of("target"));

    checker.set_seq_file(matches.is_present("seq-file"));

    checker.check_parse::<PowerPlay>("powerPlay.json")?;
    checker.check_parse::<Station>("stations.json")?;
    checker.check_parse::<SystemPopulated>("systemsPopulated.json")?;
    checker.check_parse::<SystemWithoutCoordinates>("systemsWithoutCoordinates.json")?;
    checker.check_parse::<SystemWithCoordinates>("systemsWithCoordinates.json")?;
    checker.check_parse::<Body>("bodies.json")?;

    checker.join()?;

    Ok(())
}

struct Checker<'a> {
    seq_file: bool,
    dir: &'a Path,
    check_target: Option<&'a str>,
    progresses: MultiProgress,
}

impl<'a> Checker<'a> {
    fn new(dir: &'a Path, check_target: Option<&'a str>) -> Checker<'a> {
        Checker {
            seq_file: false,
            dir,
            check_target,
            progresses: MultiProgress::new(),
        }
    }

    fn set_seq_file(&mut self, seq: bool) {
        self.seq_file = seq;
    }

    fn check_parse<D: 'static + RootEntry + Send>(&mut self, file_name: &str) -> Result<(), Error> {
        if let Some(check_target) = self.check_target {
            if check_target != file_name {
                return Ok(());
            }
        }

        let path = self.dir.join(&file_name);
        let size = path.metadata()?.len();
        let file_name = file_name.to_owned();

        if self.seq_file {
            let progress = CheckProgress(CheckProgress::new_bar(&file_name, size));

            check::<D>(path, progress, file_name).expect("check failed");
        } else {
            let progress = CheckProgress(
                self.progresses
                    .add(CheckProgress::new_bar(&file_name, size)),
            );
            spawn(|| check::<D>(path, progress, file_name).expect("check failed"));
        }

        Ok(())
    }

    fn join(&mut self) -> Result<(), Error> {
        self.progresses.join()?;
        Ok(())
    }
}

fn check<D: 'static + RootEntry + Send>(
    path: PathBuf,
    progress: CheckProgress,
    file_name: String,
) -> Result<(), Error> {
    let mut dec = ParallelDecoder::<D>::start(path, progress)?;

    while let Some(_) = dec
        .read_entry()
        .with_context(|| format!("While checking '{}'", file_name))?
    {}

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
