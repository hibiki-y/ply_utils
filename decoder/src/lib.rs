use byteorder::{ByteOrder, LittleEndian};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
extern crate parser;
use parser::PropertyType;

pub fn to_string(input_path: &str, output_path: &str) {
    const ELEMENT_VERTEX_COUNT: usize = 300;

    let mut reader = BufReader::new(File::open(input_path).expect("file open failed"));
    let mut writer = BufWriter::new(File::create(output_path).expect("file create failed"));

    let mut property_type_list = Vec::new();

    let mut header = String::new();
    while reader.read_line(&mut header).is_ok() == true {
        match header.split_ascii_whitespace().next().unwrap() {
            "format" => {
                println!("finish change format");
                header = String::from("format ascii 1.0\n");
            }
            "end_header" => {
                writer.write(header.as_bytes()).unwrap();
                println!("finish read header");
                break;
            }
            "property" => {
                property_type_list.push(PropertyType::get_type(
                    header.split_ascii_whitespace().nth(1).unwrap(),
                ));
            }
            _ => println!("reading"),
        };
        writer.write(header.as_bytes()).unwrap();
        header.clear();
    }

    for _ in 0..ELEMENT_VERTEX_COUNT {
        let mut stack_line = String::new();
        property_type_list
            .iter()
            .filter_map(|&p| p)
            .for_each(|p| match p {
                PropertyType::CHAR => println!("not implemented"),
                PropertyType::UCHAR => {
                    let mut buf: [u8; 1] = [0; 1];
                    reader.read(&mut buf).expect("can't read");
                    stack_line.push_str(buf[0].to_string().as_str());
                    stack_line.push_str(" ");
                }
                PropertyType::SHORT => println!("not implemented"),
                PropertyType::USHORT => println!("not implemented"),
                PropertyType::INT => println!("not implemented"),
                PropertyType::UINT => println!("not implemented"),
                PropertyType::FLOAT => {
                    let mut buf: [u8; 4] = [0; 4];
                    reader.read(&mut buf).expect("can't read");
                    let value = LittleEndian::read_f32(&buf);
                    stack_line.push_str(value.to_string().as_str());
                    stack_line.push_str(" ");
                }
                PropertyType::DOUBLE => println!("not implemented"),
            });
        stack_line.pop(); //delete last space
        stack_line.push_str("\n");
        writer.write(stack_line.as_bytes()).expect("can't write");
        stack_line.clear();
    }
}

#[cfg(test)]
mod tests {
    use crate::to_string;
    #[test]
    fn test_decode() {
        to_string(
            "../original/dancer_vox11_00000001.ply",
            "../out/decode_test1.ply",
        );
        // to_string(
        //     "../original/basketball_player_vox11_00000001.ply",
        //     "../out/decode_test2.ply",
        // )
    }
}
