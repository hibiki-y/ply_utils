use parser::PropertyType;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use std::time::Instant;

pub fn to_string(input_path: PathBuf, output_path: PathBuf) {
    let mut reader = BufReader::new(File::open(input_path).expect("file open failed"));
    let mut writer = BufWriter::new(File::create(output_path).expect("file create failed"));

    let mut property_type_list = Vec::new();

    let mut element_vertex_count = Vec::new();

    let mut header = String::new();
    let istart = Instant::now();
    while reader.read_line(&mut header).is_ok() {
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
            "element" => match header.split_ascii_whitespace().nth(1).unwrap() {
                "vertex" => {
                    element_vertex_count.push(
                        header
                            .split_ascii_whitespace()
                            .nth(2)
                            .unwrap()
                            .parse::<i32>()
                            .unwrap(),
                    );
                }
                _ => {
                    println!("unknown element"); //FIXME
                }
            },
            _ => println!("reading"),
        };
        writer.write(header.as_bytes()).unwrap();
        header.clear();
    }
    println!("{:?}", element_vertex_count);
    let istop = Instant::now();
    println!("header load time = {:?}", istop.duration_since(istart));

    for _ in 0..element_vertex_count.pop().unwrap() {
        let mut stack_line = String::new();
        property_type_list
            .iter()
            .filter_map(|&p| p)
            .for_each(|p| match p {
                PropertyType::CHAR => println!("not implemented"),
                PropertyType::UCHAR => {
                    let mut buf: [u8; 1] = [0; 1];
                    reader.read_exact(&mut buf).expect("can't read");
                    stack_line.push_str(buf[0].to_string().as_str());
                    stack_line.push_str(" ");
                }
                PropertyType::SHORT => println!("not implemented"),
                PropertyType::USHORT => println!("not implemented"),
                PropertyType::INT => println!("not implemented"),
                PropertyType::UINT => println!("not implemented"),
                PropertyType::FLOAT => {
                    let mut buf: [u8; 4] = [0; 4];
                    reader.read_exact(&mut buf).expect("can't read");
                    let value = f32::from_le_bytes(buf);
                    // let value = LittleEndian::read_f32(&buf);
                    stack_line.push_str(value.to_string().as_str());
                    stack_line.push_str(" ");
                }
                PropertyType::DOUBLE => println!("not implemented"),
            });
        stack_line.pop(); //delete last space
        stack_line.push_str("\n");
        writer.write(stack_line.as_bytes()).expect("can't write");
    }
    let stop = Instant::now();
    println!("payload time = {:?}", stop.duration_since(istop));
}

#[cfg(test)]
mod tests {
    use crate::to_string;
    use std::{path::PathBuf, str::FromStr};
    #[test]
    fn test_decode() {
        to_string(
            PathBuf::from_str("../original/dancer_vox11_00000001.ply").unwrap(),
            PathBuf::from("../out/decode_test1.ply"),
        );
    }
    #[test]
    fn test_another() {
        to_string(
            PathBuf::from("../original/basketball_player_vox11_00000001.ply"),
            PathBuf::from("../out/decode_test2.ply"),
        )
    }
}
