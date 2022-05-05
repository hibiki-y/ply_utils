extern crate clap;
use structopt::StructOpt;

///オプション
#[derive(StructOpt, Debug)]
pub enum METHOD {
    ///Cut ply property: cargo run -- cut -h
    CUT {
        /// TEST
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
