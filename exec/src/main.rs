extern crate cutter;
extern crate decoder;
extern crate parser;
use structopt::StructOpt;

fn main() {
    let option = parser::METHOD::from_args();
    match option {
        parser::METHOD::CUT { cut } => {
            cutter::cut_property("./test/test.ply", "./out/output_test-1.ply", 0, cut);
        }
        parser::METHOD::DECODE { decode } => {
            decoder::to_string(
                "./original/dancer_vox11_00000001.ply",
                "./out/decode_test11.ply",
            );
        }
        _ => {
            println!("not implemented");
        }
    }

    // }
    // let c = opts.cut;
    // println!("cut start");
    // cutter::cut_property("./test/test.ply", "./out/output_test-1.ply", 0, c);
    // println!("decode start");
    // decoder::to_string(
    //     "./original/dancer_vox11_00000001.ply",
    //     "./out/decode_test11.ply",
    // );
}
