use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

///read file and return write line
fn parser(mut header: String, prop: &Vec<String>) -> (String, Option<bool>) {
    // let mut header = String::new();

    let mut property_count = None;

    let h = match header.split_ascii_whitespace().next().unwrap() {
        "element" => {
            let number = header.split_ascii_whitespace().nth(2).unwrap();
            println!("point count: {}", number);
            header + "\n"
        }
        "format" => header.to_string() + "comment cut_by_ply-utils 7318184@alumni.tus.ac.jp\n",
        "property" => {
            let mut a = String::new(); //FIXME
            if prop
                .iter()
                .any(|p| p.as_str() == header.split_ascii_whitespace().nth(2).unwrap())
                == true
            {
                property_count = Some(true);
                a = header + "\n"
            } else {
                property_count = Some(false);
                header.clear()
            }
            a
        }
        "comment" => {
            header.clear();
            header
        }
        "end_header" => {
            println!("finish read header");
            header.clear();
            header
        }
        _ => header + "\n",
    };
    (h, property_count)
}
fn check(a: String) -> Option<String> {
    if a == "end_header" {
        None
    } else {
        Some(a)
    }
}

pub fn cut_auto(input_path: &str, output_path: &str, prop: Vec<String>) {
    let reader = BufReader::new(File::open(input_path).expect("file open failed"));
    let mut writer = BufWriter::new(File::create(output_path).expect("file create failed"));

    let mut property_flag = Vec::new();
    let mut reader_iter = reader.lines().into_iter();

    reader_iter
        .by_ref()
        .scan((), |_, str| check(str.unwrap()))
        .for_each(|line| {
            let (l, ps) = parser(line, &prop);
            property_flag.push(ps);
            writer.write(l.as_bytes()).unwrap();
        });
    writer.write(b"end_header\n").unwrap();
    let property_stock: Vec<(usize, bool)> = property_flag
        .iter()
        .filter_map(|&p| p)
        .enumerate()
        .collect();

    let mut buf = String::new();
    reader_iter.for_each(|x| {
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
            .filter(|(number, _)| *number < a + 3 || b <= *number && *number < b + 3)
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
        cut_property("../test/test.ply", "../out/fastcut_test-1.ply", 0, 3); //a part of Thaidancer
        cut_property("../test/test2.ply", "../out/fastcut_test2-1.ply", 0, 6); //a part of boxer
    }
    #[test]
    fn t() {
        cut_auto(
            "../test/test.ply",
            "../out/autocut_test-3.ply",
            vec![String::from("x"), String::from("ny")],
        );
    }
}
