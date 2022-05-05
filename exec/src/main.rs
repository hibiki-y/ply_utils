extern crate cutter;
extern crate decoder;
extern crate parser;
use structopt::StructOpt;

fn main() {
    let option = parser::METHOD::from_args();
    match option {
        parser::METHOD::CUT { cutsize, fast } => {
            if fast == true {
                println!("fast cut");
                cutter::cut_property("./test/test.ply", "./out/output_test-1.ply", 0, cutsize);
            } else {
                println!("not implemented yet");
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
        _ => {
            println!("not implemented");
        }
    }
}
