use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};
use reqwest::header::{HeaderMap, ETAG, IF_NONE_MATCH, USER_AGENT};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_writer_pretty};
use tiny_fail::{ErrorMessageExt, Fail};

const TIMEOUT_SECS: u64 = 10;
const BAR_TICK_SIZE: u64 = 32 * 1024;

pub struct Downloader {
    head_client: Client,
    get_client: Client,
    etags: EtagStoreage,
}

impl Downloader {
    pub fn new(etags: EtagStoreage) -> Result<Downloader, Fail> {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            USER_AGENT,
            format!(
                "EDSM Dumps Downloader/{}",
                option_env!("CARGO_PKG_VERSION").unwrap_or("unknown version")
            )
            .parse()
            .unwrap(),
        );

        let get_client = Client::builder()
            .default_headers(default_headers.clone())
            .connect_timeout(Some(Duration::from_secs(TIMEOUT_SECS)))
            .gzip(true)
            .build()?;

        let head_client = Client::builder()
            .default_headers(default_headers)
            .connect_timeout(Some(Duration::from_secs(TIMEOUT_SECS)))
            .gzip(false)
            .build()?;

        Ok(Downloader {
            get_client,
            head_client,
            etags,
        })
    }

    pub fn download(&self, target: &Target) -> Result<(), Fail> {
        // read size and update check
        let mut req = self.head_client.head(target.url());

        if let Some(etag) = self.etags.get(target)? {
            req = req.header(IF_NONE_MATCH, etag);
        }

        let res = req.send()?;

        if res.status().as_u16() == 304 {
            println!("{}: No update.", target.name()?);
            return Ok(());
        }

        let res = res.error_for_status()?;
        let size = res.content_length();

        // download
        let req = self.get_client.get(target.url());
        let mut res = req.send()?.error_for_status()?;

        let f = File::create(target.name()?)?;
        let mut w = ProgressWriter::new(f, size, target.name()?);

        res.copy_to(&mut w)?;

        w.flush()?;
        w.done();

        // save ETag
        if let Some(etag) = res.headers().get(ETAG) {
            let etag = etag.to_str().err_msg("can't parse ETag as string")?;
            self.etags.save(target, etag)?;
        } else {
            self.etags.remove(target)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
struct ProgressWriter<W: Write> {
    inner: BufWriter<W>,
    bar: ProgressBar,
}

impl<W: Write> ProgressWriter<W> {
    fn new(inner: W, size: Option<u64>, name: &str) -> ProgressWriter<W> {
        let bar = if let Some(size) = size {
            let bar = ProgressBar::new(size);
            bar.set_style(ProgressStyle::default_bar().template("{msg} [{bar:40.white/black}] {bytes}/{total_bytes}, {bytes_per_sec}, {eta_precise}"));
            bar
        } else {
            let bar = ProgressBar::new_spinner();
            bar.set_style(ProgressStyle::default_spinner().template("{spinner} {msg}"));
            bar
        };

        bar.set_draw_delta(BAR_TICK_SIZE);
        bar.set_message(name);

        ProgressWriter {
            inner: BufWriter::new(inner),
            bar,
        }
    }

    fn done(self) {
        self.bar.finish();
    }
}

impl<W: Write> Write for ProgressWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = self.inner.write(buf)?;
        self.bar.inc(n as u64);
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    url: String,
}

impl Target {
    pub fn new(url: String) -> Target {
        Target { url }
    }

    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    pub fn name(&self) -> Result<&str, Fail> {
        self.url()
            .split('/')
            .last()
            .err_msg("target URL should have name part, but not")
    }
}

#[derive(Debug, Clone)]
pub struct EtagStoreage {
    path: PathBuf,
}

impl EtagStoreage {
    pub fn new<P: AsRef<Path>>(path: P) -> EtagStoreage {
        EtagStoreage {
            path: path.as_ref().to_owned(),
        }
    }

    pub fn get(&self, target: &Target) -> Result<Option<String>, Fail> {
        if self.path.exists() {
            let f = File::open(&self.path).err_msg(format!("can't open file: {:?}", self.path))?;
            let mut table: BTreeMap<String, String> =
                from_reader(f).err_msg("can't parse ETag file")?;

            Ok(table.remove(target.url()))
        } else {
            Ok(None)
        }
    }

    pub fn save(&self, target: &Target, etag: &str) -> Result<(), Fail> {
        let mut table: BTreeMap<String, String> = if self.path.exists() {
            let f = File::open(&self.path).err_msg(format!("can't open file: {:?}", self.path))?;
            from_reader(f).err_msg("can't parse ETag file")?
        } else {
            BTreeMap::new()
        };

        table.insert(target.url().to_owned(), etag.to_owned());

        let mut f =
            File::create(&self.path).err_msg(format!("can't create file: {:?}", self.path))?;
        to_writer_pretty(&mut f, &table).err_msg("can't encode ETag file")?;

        Ok(())
    }

    pub fn remove(&self, target: &Target) -> Result<(), Fail> {
        let mut table: BTreeMap<String, String> = if self.path.exists() {
            let f = File::open(&self.path).err_msg(format!("can't open file: {:?}", self.path))?;
            from_reader(f).err_msg("can't parse ETag file")?
        } else {
            BTreeMap::new()
        };

        table.remove(target.url());

        let mut f =
            File::create(&self.path).err_msg(format!("can't create file: {:?}", self.path))?;
        to_writer_pretty(&mut f, &table).err_msg("can't encode ETag file")?;

        Ok(())
    }
}
