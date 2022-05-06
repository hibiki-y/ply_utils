extern crate cutter;
extern crate decoder;
extern crate parser;
use structopt::StructOpt;

fn main() {
    let option = parser::METHOD::from_args();
    match option {
        parser::METHOD::CUT {
            property,
            input_path,
            output_path,
        } => {
            //"./test/original_test.ply"
            println!("cut auto");
            println!("property: {:?}", property);
            cutter::cut_auto(input_path, output_path, property);
            println!("cut auto end");
        }
        parser::METHOD::DECODE {
            switch,
            input_path,
            output_path,
        } => {
            if switch == true {
                decoder::to_string(input_path, output_path);
            } else {
                println!("not implemented");
            }
        }
    }
}
