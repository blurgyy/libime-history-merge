use std::path::PathBuf;

use libime_history_merge::{data::History, merge, Error, Result};
use structopt::StructOpt;

/// Inspect/Merge one or more `user.history` files.
#[derive(Debug, StructOpt)]
#[structopt(
    global_settings(&[structopt::clap::AppSettings::ColoredHelp]),
)]
pub struct Opt {
    /// Path to a history file, in binary or in plain-text.
    pub user_history_path: PathBuf,

    /// More history files.
    pub more_paths: Vec<PathBuf>,

    /// A list of integer values (e.g. "-w3,5" or "-w 4 1"), represents relative weights assigned
    /// to each of the input history data while merging, sum of weights are normalized to 1.
    #[structopt(short, long, use_delimiter = true)]
    pub weights: Vec<u8>,

    /// If present, write merged history data to specified path;  If not present, inspect the
    /// merged history data in plain text.
    #[structopt(short, long)]
    pub output: Option<PathBuf>,

    /// If present, let the user edit the output history.
    #[structopt(short, long)]
    pub edit: bool,

    /// If present, do not invoke a pager (pager defaults to the environment variable $PAGER's
    /// value).
    #[structopt(short, long)]
    pub no_pager: bool,
}

fn setup() -> Opt {
    // Suppress "Broken pipe" error when piping stdout to a pager and not scrolling to the bottom.
    // REF: <https://github.com/rust-lang/rust/issues/46016#issuecomment-428106774>
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
    std::env::set_var(
        "RUST_LOG",
        std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
    );
    pretty_env_logger::init();

    Opt::from_args()
}

fn run() -> Result<()> {
    let mut opts = setup();

    let mut histories = vec![opts.user_history_path];
    histories.append(&mut opts.more_paths);
    let histories: Vec<History> = histories.iter().map(History::load).collect::<Result<_>>()?;

    let merged = merge(histories, opts.weights)?;

    match opts.output {
        Some(path) => {
            if path.exists() {
                return Err(Error::IoError("Output path already exists".to_string()));
            }
            let merged = if opts.edit {
                History::load_from_text(edit::edit(merged.to_string())?.as_bytes())?
            } else {
                merged
            };
            merged.save(&path)?;
        }
        None => {
            if opts.edit {
                return Err(Error::LogicError(
                    "-o|--output is not specified, the edited history will be lost".to_string(),
                ));
            }
            if !opts.no_pager && opts.output.is_none() {
                pager::Pager::with_default_pager("less").setup();
            }
            println!("{}", merged);
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        log::error!("{}", e);
        std::process::exit(1);
    }
}

// Author: Blurgy <gy@blurgy.xyz>
// Date:   Feb 03 2022, 16:33 [CST]
