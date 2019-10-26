
use tiny_fail::{ErrorMessageExt, Fail};

use edsm_dumps_downloader::download::{Downloader, EtagStoreage};

fn main() {
    if let Err(fail) = w_main() {
        eprintln!("Error: {}", fail);
        std::process::exit(1);
    }
}

fn w_main() -> Result<(), Fail> {
    let etags = EtagStoreage::new("./.etags.json");
    let _dl = Downloader::new(etags).err_msg("can't load download targets")?;

    Ok(())
}
