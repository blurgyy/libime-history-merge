use std::{
    fs::Permissions, os::unix::prelude::PermissionsExt, path::PathBuf,
};

use libime_history_merge::{data::History, to_bytes, Error, Result};
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

    /// If present, merge given histories into if not present, inspect the
    /// merged history in plain text.
    #[structopt(short, long)]
    pub output: Option<PathBuf>,

    /// If present, do not invoke a pager
    #[structopt(short, long)]
    pub no_pager: bool,
}

fn merge(histories: Vec<History>) -> Result<History> {
    let mut pools = Vec::new();

    for hist in histories {
        pools.append(&mut hist.pools.to_owned());
    }

    Ok(History::new(pools))
}

fn main() -> Result<()> {
    // Suppress "Broken pipe" error when piping stdout to a pager and not
    // scrolling to the bottom.  Below snippet is taken from this link:
    // https://github.com/rust-lang/rust/issues/46016#issuecomment-428106774
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }

    let mut opts = Opt::from_args();

    if !opts.no_pager {
        pager::Pager::with_default_pager("less").setup();
    }

    let mut histories = vec![opts.user_history_path];
    histories.append(&mut opts.more_paths);
    let histories: Vec<History> = histories
        .iter()
        .map(|hist_path| History::load(hist_path).ok().unwrap())
        .collect();

    let merged = merge(histories)?;

    match opts.output {
        Some(path) => {
            if path.exists() {
                return Err(Error::IoError(
                    "Target path already exists".to_owned(),
                ));
            }
            std::fs::write(&path, to_bytes(&merged)?)?;
            std::fs::set_permissions(path, Permissions::from_mode(0o600))?;
        }
        None => {
            println!("{}", merged);
        }
    }

    Ok(())
}

// Author: Blurgy <gy@blurgy.xyz>
// Date:   Feb 03 2022, 16:33 [CST]
