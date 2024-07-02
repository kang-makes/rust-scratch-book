use lazy_static::lazy_static;
use std::collections::HashSet;
use std::error::Error as StdError;
use std::io::Result as IoResult;
use std::io::Write;
use std::io::{BufRead, BufReader, BufWriter};
use std::io::{Error as IoError, ErrorKind};
use std::sync::RwLock;

use tracing::Level;
use tracing::{debug, info, warn};
use tracing_subscriber::fmt::format::PrettyFields;
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

const BUFFER_SIZE: usize = 5120;

lazy_static! {
    static ref secrets: RwLock<HashSet<String>> = RwLock::new(HashSet::new());
}

fn insert_secret(secret: impl ToString) -> Result<(), Box<dyn StdError>> {
    let mut write_lock = secrets.write()?;

    // TODO: What if a license Key is inserted twice?
    // We need here an ARC or a hashmap<string, i32>
    (*write_lock).insert(secret.to_string());

    Ok(())
}

fn delete_secret(secret: impl ToString) -> Result<(), Box<dyn StdError>> {
    let mut write_lock = secrets.write()?;

    (*write_lock).remove(&secret.to_string());

    Ok(())
}

#[derive(Debug)]
pub(crate) struct CensorWriter<W>
where
    W: Write,
{
    to: BufWriter<W>,
}

impl<W> CensorWriter<W>
where
    W: Write,
{
    pub(crate) fn new(writer: W) -> Self {
        Self {
            to: BufWriter::with_capacity(BUFFER_SIZE, writer),
        }
    }
}

// IMPLEMENTATION OF io::Write
impl<W> Write for CensorWriter<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        let mut reader = BufReader::with_capacity(buf.len(), buf);
        let mut line = String::new();

        let result = reader.read_line(&mut line)?;

        let censors = secrets.read().map_err(|e| {
            IoError::new(
                ErrorKind::Other,
                format!("error while getting censors lock: {}", e),
            )
        })?;

        for censor in &*censors {
            line = line.replace(censor.as_str(), "SENSITIVE DATA");
        }

        self.to.write_all(line.as_bytes())?;
        Ok(result)
    }

    fn flush(&mut self) -> IoResult<()> {
        todo!()
    }
}

fn main() -> Result<(), Box<dyn StdError>> {
    let console_layer = tracing_subscriber::fmt::layer()
        .with_writer(|| CensorWriter::new(std::io::stdout()))
        .with_timer(ChronoLocal::new("".into()))
        .fmt_fields(PrettyFields::new())
        .with_filter(
            EnvFilter::builder()
                .with_default_directive(Level::DEBUG.into())
                .with_env_var("LOG_LEVEL")
                .from_env_lossy(),
        );

    tracing_subscriber::Registry::default()
        .with(console_layer)
        .try_init()?;

    warn!("Logger initialized!");

    let secret = "secretize-me-please";

    info!("Printing secret as explicit: {}", secret);
    debug!("Printing secret as debug: {:?}", secret);
    let _ = insert_secret(secret);
    info!("Printing secret as explicit: {}", secret);
    debug!("Printing secret as debug: {:?}", secret);
    let _ = delete_secret(secret);
    info!("Printing secret as explicit: {}", secret);
    debug!("Printing secret as debug: {:?}", secret);

    Ok(())
}
