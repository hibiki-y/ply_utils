use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
extern crate decoder;
use decoder::CUT_TYPE_RANGE;
use decoder::HEADER_CAPACITY;
extern crate parser;

pub fn cut_auto(input_path: &str, output_path: &str, prop: Vec<String>) {
    let mut reader = BufReader::new(File::open(input_path).expect("file open failed"));
    let mut writer = BufWriter::new(File::create(output_path).expect("file create failed"));
    let mut header = String::new();

    let mut point_count = 0;
    let mut property_stock = Vec::new();
    let mut property_count = 0usize;
    println!("cut start");
    for _ in 1..HEADER_CAPACITY {
        reader.read_line(&mut header).expect("reading fail");
        match header.split_ascii_whitespace().next().unwrap() {
            "element" => {
                let number = header.split_ascii_whitespace().nth(2).unwrap();
                point_count = number.parse::<u32>().unwrap();
                println!("point count: {}", point_count);
                writer.write(header.as_bytes()).unwrap();
                header.clear();
            }
            "format" => {
                writer.write(header.as_bytes()).unwrap();
                writer
                    .write(b"comment cut_by_ply-utils 7318184@alumni.tus.ac.jp\n")
                    .unwrap();
                header.clear();
            }
            "property" => {
                if prop
                    .iter()
                    .any(|p| p.as_str() == header.split_ascii_whitespace().nth(2).unwrap())
                    == true
                {
                    property_stock.push(property_count);
                    writer.write(header.as_str().as_bytes()).unwrap();
                    header.clear();
                } else {
                    header.clear();
                }

                property_count += 1;
            }
            "comment" => {
                header.clear();
            }
            "end_header" => {
                writer.write(header.as_bytes()).unwrap();
                header.clear();
                println!("finish read header");
                break;
            }
            _ => {
                writer.write(header.as_bytes()).unwrap();
                header.clear();
            }
        }
    }
    let mut buf = String::new();
    reader.lines().into_iter().for_each(|x| {
        x.unwrap()
            .split_ascii_whitespace()
            .enumerate()
            .filter(|(number, _)| property_stock.iter().any(|i| i == number))
            .for_each(|(_, word)| {
                buf.push_str(word);
                buf.push_str(" ");
            });
        buf.pop(); //delete last space
        buf.push_str("\n");
    });
    writer.write(buf.as_bytes()).unwrap();
}

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
    use crate::cut_auto;
    use crate::cut_property;
    #[test]
    fn test_cut() {
        cut_property("../test/test.ply", "../out/output_test-1.ply", 0, 3); //a part of Thaidancer
        cut_property("../test/test2.ply", "../out/output_test2-1.ply", 0, 6); //a part of boxer
    }
    #[test]
    fn t() {
        cut_auto(
            "../test/test.ply",
            "../out/output_test-3.ply",
            vec![String::from("x"), String::from("ny")],
        );
    }
}
