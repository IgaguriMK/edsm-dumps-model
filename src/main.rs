use std::io::{stdin, stdout, Write, BufRead};

use tiny_fail::{ErrorMessageExt, Fail};

use edsm_dumps_downloader::download::Downloader;
use edsm_dumps_downloader::target::{EtagStoreage, Mode, Target};

fn main() {
    if let Err(fail) = w_main() {
        eprintln!("Error: {}", fail);
        std::process::exit(1);
    }
}

fn w_main() -> Result<(), Fail> {
    let targets = Target::load_list("./targets.json").err_msg("can't load download targets")?;
    let etags = EtagStoreage::new("./.caches.json");
    let dl = Downloader::new(etags)?;

    let mode = read_mode()?;

    for target in &targets {
        if target.mode() > mode {
            continue;
        }

        dl.download(target)?;
    }

    Ok(())
}

fn read_mode() -> Result<Mode, Fail> {
    let out = stdout();
    let mut out_lock = out.lock();

    let stdin = stdin();
    let mut in_lock = stdin.lock();
    let mut buf_str = String::new();

    loop {
        write!(&mut out_lock, "Enter mode (small / normal / full): ")?;
        out_lock.flush()?;
        in_lock.read_line(&mut buf_str)?;

        match Mode::parse(&buf_str) {
            Ok(mode) => return Ok(mode),
            Err(e) => writeln!(&mut out_lock, "{}", e)?,
        }
    }
}