use structopt::StructOpt;

mod opt;
mod server;
mod client;

use opt::PiOpts;
use server::server;
use client::client;

fn main() {
    let opt = PiOpts::from_args();
    if opt.debug {
        println!("{:?}", opt);
    }

    if opt.server {
        server(opt);
    } else {
        client(opt);
    }
}
