extern crate clap;
use structopt::StructOpt;

///コマンドライン引数
#[derive(StructOpt, Debug)]
pub enum METHOD {
    ///Cut ply property: cargo run -- cut -h
    CUT {
        /// TEST
        #[structopt(short, long, help = "put cut size")]
        cutsize: usize,
        #[structopt(short, long, help = "put cut property")]
        property: Vec<String>,
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
pub enum PROPERTY {
    X,
    Y,
    Z,
    NX,
    NY,
    NZ,
    RED,
    GREEN,
    BLUE,
    ALPHA,
    MATERIALINDEX,
}
impl PROPERTY {
    pub fn parse_property(s: &str) -> PROPERTY {
        match s {
            "x" => PROPERTY::X,
            "y" => PROPERTY::Y,
            "z" => PROPERTY::Z,
            "nx" => PROPERTY::NX,
            "ny" => PROPERTY::NY,
            "nz" => PROPERTY::NZ,
            "red" => PROPERTY::RED,
            "green" => PROPERTY::GREEN,
            "blue" => PROPERTY::BLUE,
            _ => unreachable!(),
        }
    }
}
