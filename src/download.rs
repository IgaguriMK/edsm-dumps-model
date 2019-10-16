use std::time::Duration;
use std::fmt;
use std::fs::File;
use std::io::{self, Write, BufWriter};

use progress::{Bar, SpinningCircle};
use reqwest::header::{HeaderMap, IF_NONE_MATCH, USER_AGENT, ETAG};
use reqwest::{Client};
use tiny_fail::{ErrorMessageExt, Fail};

use crate::target::{EtagStoreage, Target};

const TIMEOUT_SECS: u64 = 10;
const PROGRESS_SIZE: usize = 1024 * 1024;

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

        Ok(Downloader { get_client, head_client, etags })
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
    progress: Progress,
}

impl <W: Write> ProgressWriter<W> {
    fn new(inner: W, size: Option<u64>, name: &str) -> ProgressWriter<W> {
        ProgressWriter {
            inner: BufWriter::new(inner),
            progress: Progress::new(size, name),
        }
    }

    fn done(self) {
        self.progress.done();
    }
}

impl <W: Write> Write for ProgressWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = self.inner.write(buf)?;
        self.progress.add(n);
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

enum Progress {
    Bar{bar: Bar, current: usize, total: usize, percent: i32},
    Spin{spin: SpinningCircle, current: usize},
}

impl Progress {
    fn new(size: Option<u64>, name: &str) -> Progress {
        match size {
            Some(size) => {
                let mut bar = Bar::new();
                bar.set_job_title(name);
                Progress::Bar{
                    bar,
                    current: 0,
                    total: size as usize,
                    percent: 0,
                }
            } ,
            None => {
                let mut spin = SpinningCircle::new();
                spin.set_job_title(name);
                Progress::Spin {
                    spin,
                    current: 0,
                }
            }
        }
    }

    fn add(&mut self, amt: usize) {
        match self {
            Progress::Bar{bar, current, total, percent} => {
                *current += amt;
                let r = (*current as f64) / (*total as f64);
                let p = (100.0 * r) as i32;
                if p != *percent {
                    bar.reach_percent(p);
                    *percent = p;
                }
            }
            Progress::Spin{spin, current} => {
                *current += amt;
                let n = *current / PROGRESS_SIZE;
                *current %= PROGRESS_SIZE;
                for _ in 0..n {
                    spin.tick();
                }
            }
        }
    }

    fn done(self) {
        match self {
            Progress::Bar{mut bar, ..} => bar.jobs_done(),
            Progress::Spin{spin, ..} => spin.jobs_done(),
        }
    }
}

impl fmt::Debug for Progress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Progress::Bar{..} => write!(f, "Progress::Bar"),
            Progress::Spin{..} => write!(f, "Progress::Spin"),
        }
    }
}

