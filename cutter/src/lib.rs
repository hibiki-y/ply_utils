use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
extern crate parser;
use parser::PARSER;
use std::path::PathBuf;

pub fn cut_auto(input_path: PathBuf, output_path: PathBuf, property: Vec<String>) {
    let mut writer = BufWriter::new(File::create(output_path).expect("file create failed"));
    let mut reader_iter = BufReader::new(File::open(input_path).expect("file open failed"))
        .lines()
        .into_iter();

    let mut property_flag = Vec::new(); //propertyならSome
    reader_iter
        .by_ref()
        .scan((), |_, str| PARSER::check_end_header(str.unwrap()))
        .for_each(|line| {
            let (l, flag) = PARSER::parser(line, &property);
            property_flag.push(flag);
            writer.write(l.as_bytes()).unwrap();
        });
    writer.write(b"end_header\n").unwrap();

    reader_iter.for_each(|x| {
        let mut buf = String::new();
        let property_stock: Vec<(usize, bool)> = property_flag
            .iter()
            .filter_map(|&p| p)
            .enumerate()
            .collect();
        x.unwrap()
            .split_ascii_whitespace()
            .enumerate()
            .filter(|&(number, _)| {
                property_stock
                    .iter()
                    .any(|&(pos, flag)| pos == number && flag == true)
            })
            .for_each(|(_, word)| {
                buf.push_str(word);
                buf.push_str(" ");
            });
        buf.pop(); //delete last space
        buf.push_str("\n");
        writer.write(buf.as_bytes()).unwrap();
        buf.clear();
    });
}

#[cfg(test)]
mod tests {
    use crate::cut_auto;
    use std::path::PathBuf;
    #[test]
    fn test() {
        cut_auto(
            PathBuf::from("../test/original_test.ply"),
            PathBuf::from("../out/cut_auto_test.ply"),
            vec![String::from("x"), String::from("ny")],
        );
    }
}
