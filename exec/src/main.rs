extern crate cutter;
extern crate decoder;
extern crate parser;
use structopt::StructOpt;

fn main() {
    let option = parser::METHOD::from_args();
    match option {
        parser::METHOD::CUT { fast, property } => {
            if fast == true {
                println!("not implemented");
            } else {
                println!("cut auto");
                println!("property: {:?}", property);
                cutter::cut_auto("./test/original_test.ply", "./out/result.ply", property);
            }
        }
        parser::METHOD::DECODE { switch } => {
            if switch == true {
                decoder::to_string(
                    "./original/dancer_vox11_00000001.ply",
                    "./out/decode_test11.ply",
                );
            } else {
                println!("exec is false");
            }
        }
    }
}
