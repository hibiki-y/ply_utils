extern crate clap;
use structopt::StructOpt;

///コマンドライン引数
#[derive(StructOpt, Debug)]
#[structopt(about = "I am a program and I work, just pass `-h`")]
pub enum METHOD {
    CUT {
        /// TEST
        // #[structopt(short = "c", long = "cut")]
        cut: usize,
    },
    DECODE {
        /// TEST
        #[structopt(short = "d", long = "decode")]
        decode: bool,
    },
}
// #[structopt(name = "basic")]
