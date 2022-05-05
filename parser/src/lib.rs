extern crate clap;
use structopt::StructOpt;

///コマンドライン引数
#[derive(StructOpt, Debug)]
pub enum METHOD {
    ///Cut ply property: cargo run -- cut -h
    CUT {
        /// TEST
        #[structopt(short = "c", long = "cut", help = "put cut size")]
        cutsize: usize,
        #[structopt(short, long, help = "Put command is true")]
        fast: bool,
    },
    ///Decode ply property: cargo run -- decode -h
    DECODE {
        /// TEST
        #[structopt(short, long, help = "put command is true")]
        switch: bool,
    },
    //Example: cargo run -- decode -s 1
}
