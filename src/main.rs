use std::{
    fs::Permissions, os::unix::prelude::PermissionsExt, path::PathBuf,
};

use libime_history_merge::{data::History, merge, to_bytes, Error, Result};
use structopt::StructOpt;

/// Inspect/Merge one or more `user.history` files.
#[derive(Debug, StructOpt)]
#[structopt(
    global_settings(&[structopt::clap::AppSettings::ColoredHelp]),
)]
pub struct Opt {
    /// Path to a binary `user.history` file.
    pub user_history_path: PathBuf,

    /// More `user.history` files.
    pub more_paths: Vec<PathBuf>,

    /// A list of integer values (e.g. "-w3,5" or "-w 4 1"), represents
    /// relative weights assigned to each of the input history data while
    /// merging, sum of weights are normalized to 1.
    #[structopt(short, long, use_delimiter = true)]
    pub weights: Vec<usize>,

    /// If present, merge given history data into one;  If not present,
    /// inspect the merged history data in plain text.
    #[structopt(short, long)]
    pub output: Option<PathBuf>,

    /// If present, do not invoke a pager (pager defaults to the environment
    /// variable $PAGER's value)
    #[structopt(short, long)]
    pub no_pager: bool,
}

fn setup() -> Opt {
    // Suppress "Broken pipe" error when piping stdout to a pager and not
    // scrolling to the bottom.  Below snippet is taken from this link:
    // https://github.com/rust-lang/rust/issues/46016#issuecomment-428106774
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
    std::env::set_var(
        "RUST_LOG",
        std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
    );
    pretty_env_logger::init();
    let opts = Opt::from_args();

    opts
}

fn run() -> Result<()> {
    let mut opts = setup();

    let mut histories = vec![opts.user_history_path];
    histories.append(&mut opts.more_paths);
    let histories: Vec<History> = histories
        .iter()
        .map(|hist_path| History::load(hist_path))
        .collect::<Result<_>>()?;

    let merged = merge(histories, opts.weights)?;

    match opts.output {
        Some(path) => {
            if path.exists() {
                return Err(Error::IoError(
                    "Output path already exists".to_string(),
                ));
            }
            std::fs::write(&path, to_bytes(&merged)?)?;
            std::fs::set_permissions(path, Permissions::from_mode(0o600))?;
        }
        None => {
            if !opts.no_pager && opts.output.is_none() {
                pager::Pager::with_default_pager("less").setup();
            }
            println!("{}", merged);
        }
    }

    Ok(())
}

fn main() {
    match run() {
        Err(e) => {
            log::error!("{}", e);
            std::process::exit(1);
        }
        Ok(_) => {}
    }
}

// Author: Blurgy <gy@blurgy.xyz>
// Date:   Feb 03 2022, 16:33 [CST]
