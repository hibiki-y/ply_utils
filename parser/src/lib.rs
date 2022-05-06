extern crate clap;
use std::path::PathBuf;
use structopt::StructOpt;

///オプション
#[derive(StructOpt, Debug)]
pub enum METHOD {
    ///Cut ply property: cargo run -- cut -h
    CUT {
        /// TEST
        #[structopt(short, long, help = "put cut property")]
        property: Vec<String>,
        #[structopt(short, long, help = "Put input_path")]
        input_path: PathBuf,
        #[structopt(short, long, help = "Put output_path")]
        output_path: PathBuf,
    },
    ///Decode ply property: cargo run -- decode -h
    DECODE {
        /// TEST
        #[structopt(short, long, help = "put command is true")]
        switch: bool,
        #[structopt(short, long, help = "Put input_path")]
        input_path: PathBuf,
        #[structopt(short, long, help = "Put output_path")]
        output_path: PathBuf,
    },
    //Example: cargo run -- decode -s 1
}
///文字列分類器
pub struct PARSER;
impl PARSER {
    ///read file and return write line
    pub fn parser(mut line: String, prop: &Vec<String>) -> (String, Option<bool>) {
        let mut property_count = None;
        let h = match line.split_ascii_whitespace().next().unwrap() {
            "element" => {
                let number = line.split_ascii_whitespace().nth(2).unwrap();
                println!("point count: {}", number);
                line + "\n"
            }
            "format" => line.to_string() + "comment cut_by_ply-utils 7318184@alumni.tus.ac.jp\n",
            "property" => {
                let mut a = String::new(); //FIXME
                if prop
                    .iter()
                    .any(|p| p.as_str() == line.split_ascii_whitespace().nth(2).unwrap())
                    == true
                {
                    property_count = Some(true);
                    a = line + "\n"
                } else {
                    property_count = Some(false);
                    line.clear()
                }
                a
            }
            "comment" => {
                line.clear();
                line
            }
            _ => line + "\n",
        };
        (h, property_count)
    }
    pub fn check_end_header(a: String) -> Option<String> {
        if a == "end_header" {
            None
        } else {
            Some(a)
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub enum PropertyType {
    ///(8 ビット) 文字
    CHAR,
    ///(8 ビット) 符号なし文字
    UCHAR,
    ///(16 ビット) short 型整数
    SHORT,
    ///(16 ビット) 符号なし short 型整数
    USHORT,
    ///(32 ビット) 整数
    INT,
    ///(32 ビット) 符号なし整数
    UINT,
    ///(32 ビット) 単精度浮動小数点
    FLOAT,
    ///(64 ビット) 倍精度浮動小数点
    DOUBLE,
}
impl PropertyType {
    pub fn get_type(a: &str) -> Option<PropertyType> {
        match a {
            "char" => Some(PropertyType::CHAR),
            "uchar" => Some(PropertyType::UCHAR),
            "short" => Some(PropertyType::SHORT),
            "ushort" => Some(PropertyType::USHORT),
            "int" => Some(PropertyType::INT),
            "uint" => Some(PropertyType::UINT),
            "float" => Some(PropertyType::FLOAT),
            "double" => Some(PropertyType::DOUBLE),
            "list" => None, //fot "property list uchar int vertex_indices"
            _ => panic!("unknown type"),
        }
    }
}
