#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate docopt;
extern crate rustc_serialize;

extern crate raft;

docopt!(Args derive Debug, "
usage:
    raft <id> <port> <peer>...
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

    raft::start(args.arg_id.parse().unwrap(),
                args.arg_port.parse().unwrap(),
                args.arg_peer);
}
