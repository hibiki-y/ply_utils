use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
extern crate decoder;
use decoder::CUT_TYPE_RANGE;

///a番目からCUT_TYPE_RANGEこ,b番目からCUT_TYPE_RANGEこを切り取る
pub fn cut_property(input_path: &str, output_path: &str, a: usize, b: usize) {
    let reader = BufReader::new(File::open(input_path).expect("file open failed"));
    let mut buf = String::new();
    let mut writer = BufWriter::new(File::create(output_path).expect("file create failed"));
    reader.lines().into_iter().for_each(|x| {
        x.unwrap()
            .split_ascii_whitespace()
            .enumerate()
            .filter(|(number, _)| {
                *number < a + decoder::CUT_TYPE_RANGE
                    || b <= *number && *number < b + CUT_TYPE_RANGE
            })
            .for_each(|(_, word)| {
                buf.push_str(word);
                buf.push_str(" ");
            });
        buf.pop(); //delete last space
        buf.push_str("\n");
    });
    writer.write(buf.as_bytes()).unwrap();
}
#[cfg(test)]
mod tests {
    use crate::cut_property;
    #[test]
    fn test_cut() {
        cut_property("../test/test.ply", "../out/output_test-1.ply", 0, 3); //a part of Thaidancer
        cut_property("../test/test2.ply", "../out/output_test2-1.ply", 0, 6); //a part of boxer
    }
}
