use std::time::Instant;
use structopt::StructOpt;

fn main() {
    let option = parser::METHOD::from_args();
    let istart = Instant::now();
    match option {
        parser::METHOD::CUT {
            property,
            input_path,
            output_path,
        } => {
            println!("cut auto");
            println!("cut property: {:?}", property);
            cutter::cut_auto(input_path, output_path, property);
            println!("cut auto end");
            println!("cut time = {:?}", istart.elapsed());
        }
        parser::METHOD::DECODE {
            switch,
            input_path,
            output_path,
        } => {
            if switch == true {
                println!("not implemented");
            } else {
                decoder::to_string(input_path, output_path);
                println!("decode time = {:?}", istart.elapsed());
            }
        }
    }
}
