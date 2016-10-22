#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate docopt;
extern crate rustc_serialize;

extern crate raft;

use std::env;
use std::process::Command;

docopt!(Args derive Debug, "
usage:
    raft init
    raft -h | --help
    raft -v | --version

options:
    -h, --help              show this usage text
    -v, --version           show version information
");

fn main() {
    let args: Args = Args::docopt()
        .version(Some(format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))))
        .decode().unwrap_or_else(|e| e.exit());

    match args {
        Args { cmd_init: true, .. } => init(),
        _ => unreachable!(),
    }
}

fn init() {
    let servercmd = env::current_dir().unwrap().join("raft-server");
    for _ in 0..raft::RAFT_CLUSTER_SIZE {
        Command::new(servercmd.as_os_str())
            .spawn().expect("unable to create raft-server");
    }
}
