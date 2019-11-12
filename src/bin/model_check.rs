use std::fs::File;
use std::io::{stderr, BufReader, Write};
use std::path::{Path, PathBuf};
use std::thread::{spawn, sleep};
use std::time::Duration;

use clap::{App, Arg};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use tiny_fail::{ErrorMessageExt, Fail};

use edsm_dumps_model::array_decoder::{ArrayDecoder, Progress};
use edsm_dumps_model::config::Config;
use edsm_dumps_model::model::body::Body;
use edsm_dumps_model::model::system::{SystemWithCoordinates, SystemWithoutCoordinates};
use edsm_dumps_model::model::system_populated::SystemPopulated;
use edsm_dumps_model::model::station::Station;
use edsm_dumps_model::model::RootEntry;
use edsm_dumps_model::model::powerplay::PowerPlay;

fn main() {
    if let Err(fail) = w_main() {
        eprintln!("Error: {}", fail);
        std::process::exit(1);
    }
}

fn w_main() -> Result<(), Fail> {
    let matches = App::new("model_checker")
        .arg(
            Arg::with_name("target")
                .short("t")
                .long("target")
                .takes_value(true)
                .help("Specify check target"),
        )
        .get_matches();

    let cfg = Config::load("./config.toml").err_msg("failed load config file")?;

    let dumps_dir = cfg.dumps_dir();
    let mut checker = Checker::new(dumps_dir.as_ref(), matches.value_of("target"));

    checker.check_parse::<Body>("bodies.json")?;
    checker.check_parse::<PowerPlay>("powerPlay.json")?;
    checker.check_parse::<Station>("stations.json")?;
    checker.check_parse::<SystemPopulated>("systemsPopulated.json")?;
    checker.check_parse::<SystemWithCoordinates>("systemsWithCoordinates.json")?;
    checker.check_parse::<SystemWithoutCoordinates>("systemsWithoutCoordinates.json")?;

    checker.join()?;

    Ok(())
}

struct Checker<'a> {
    dir: &'a Path,
    check_target: Option<&'a str>,
    progresses: MultiProgress,
}

impl<'a> Checker<'a> {
    fn new(dir: &'a Path, check_target: Option<&'a str>) -> Checker<'a> {
        Checker {
            dir,
            check_target,
            progresses: MultiProgress::new(),
        }
    }

    fn check_parse<D: RootEntry>(&mut self, file_name: &str) -> Result<(), Fail> {
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

        spawn(move || {
            if let Err(e) = check::<D>(path, progress, file_name) {
                let err_out = stderr();
                let mut err_out_lock = err_out.lock();
                writeln!(err_out_lock, "{}", e).unwrap();
                err_out_lock.flush().unwrap();
                sleep(Duration::from_millis(100));
                std::process::exit(1);
            }
        });

        Ok(())
    }

    fn join(&mut self) -> Result<(), Fail> {
        self.progresses.join()?;
        Ok(())
    }
}

fn check<D: RootEntry>(
    path: PathBuf,
    progress: CheckProgress,
    file_name: String,
) -> Result<(), Fail> {
    let f = File::open(&path).err_msg(format!("failed open dump file '{:?}'", path))?;
    let r = BufReader::new(f);
    let dec = ArrayDecoder::new(r);

    let mut dec = dec.set_progress(progress);

    while let Some(_) = dec
        .read_entry::<D>()
        .err_msg(format!("While checking '{}'", file_name))?
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
