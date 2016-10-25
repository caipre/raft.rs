#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate env_logger;
extern crate log;

extern crate docopt;
extern crate rustc_serialize;

extern crate raft;

use env_logger::LogBuilder;
use log::LogLevelFilter;

docopt!(Args derive Debug, "
usage:
    raft <port> <peer>...
    raft -h | --help
    raft -v | --version

options:
    -h, --help              show this usage text
    -v, --version           show version information
");

fn main() {
    let mut builder = LogBuilder::new();
    builder.format(|msg| format!("{}", msg.args()));
    builder.filter(None, LogLevelFilter::Debug);
    builder.init().unwrap();

    let args: Args = Args::docopt()
        .version(Some(format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))))
        .decode().unwrap_or_else(|e| e.exit());

    raft::start(args.arg_port.parse().unwrap(), args.arg_peer);
}
