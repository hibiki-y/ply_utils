extern crate clap;
use std::fs::File;
use std::io::BufReader;
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
// struct HEADER {
//     pub header: String,
//     property_stock: Vec<usize>,
// }
// impl HEADER {
//     pub fn new() -> HEADER {
//         HEADER {
//             header: String::new(),
//         }
//     }
//     pub fn parse(reader: BufReader<File>) {
//         reader.read_line(&mut header).expect("reading fail");
//         match header.split_ascii_whitespace().next().unwrap() {
//             "element" => {
//                 let number = header.split_ascii_whitespace().nth(2).unwrap();
//                 println!("point count: {}", number);
//                 writer.write(header.as_bytes()).unwrap();
//                 header.clear();
//             }
//             "format" => {
//                 writer.write(header.as_bytes()).unwrap();
//                 writer
//                     .write(b"comment cut_by_ply-utils 7318184@alumni.tus.ac.jp\n")
//                     .unwrap();
//                 header.clear();
//             }
//             "property" => {
//                 if prop
//                     .iter()
//                     .any(|p| p.as_str() == header.split_ascii_whitespace().nth(2).unwrap())
//                     == true
//                 {
//                     property_stock.push(property_count);
//                     writer.write(header.as_str().as_bytes()).unwrap();
//                     header.clear();
//                 } else {
//                     header.clear();
//                 }

//                 property_count += 1;
//             }
//             "comment" => {
//                 header.clear();
//             }
//             "end_header" => {
//                 writer.write(header.as_bytes()).unwrap();
//                 header.clear();
//                 println!("finish read header");
//                 break;
//             }
//             _ => {
//                 writer.write(header.as_bytes()).unwrap();
//                 header.clear();
//             }
//         }
//     }
// }
