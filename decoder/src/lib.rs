use byteorder::{ByteOrder, LittleEndian};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

///xyz,rgb,全て3つで1セットのため
pub const CUT_TYPE_RANGE: usize = 3;

pub const HEADER_CAPACITY: u8 = 80;

pub fn to_string(input_path: &str, output_path: &str) {
    const ELEMENT_VERTEX_COUNT: usize = 300;

    let mut reader = BufReader::new(File::open(input_path).expect("file open failed"));
    let mut writer = BufWriter::new(File::create(output_path).expect("file create failed"));
    let mut header = String::new();

    let mut point_count = 0;

    for i in 1..HEADER_CAPACITY {
        reader.read_line(&mut header).expect("reading fail");
        //FIXME: if文の処理重い
        if i == 3 {
            let number = header.split_ascii_whitespace().nth(2).unwrap();
            point_count = number.parse::<u32>().unwrap();
            println!("point count: {}", point_count);
        }
        match header.as_str() {
            "format binary_little_endian 1.0\n" => {
                let f = String::from("format ascii 1.0\n");
                writer.write(f.as_bytes()).unwrap();
                header.clear();
                println!("finish change format");
            }
            "end_header\n" => {
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
    let mut stack = String::new();
    for j in 0..ELEMENT_VERTEX_COUNT {
        for i in 0..CUT_TYPE_RANGE * 2 {
            let mut buf: [u8; 4] = [0; 4];
            // if j == 281 && i == 4 {
            //     //FIXME: ここはどうする？
            //     let mut delete: [u8; 2] = [0; 2];
            //     reader.read(&mut delete).expect("can't read");
            // }
            reader.read(&mut buf).expect("can't read");
            if j == 280 {
                println!("280 buf is {:?}", buf);
            }
            if j == 281 {
                println!("buf is {:?}", buf);
            }
            let value = LittleEndian::read_f32(&buf);
            if i == 3 {
                println!("{}行目{}", &j, value);
            }
            stack.push_str(value.to_string().as_str());
            stack.push_str(" ");
        }

        for _ in 0..CUT_TYPE_RANGE + 1 {
            let mut buf: [u8; 1] = [0; 1];
            reader.read(&mut buf).expect("can't read");
            stack.push_str(buf[0].to_string().as_str());
            stack.push_str(" ");
        }
        stack.pop(); //delete last space
        stack.push_str("\n");
        writer.write(stack.as_bytes()).expect("can't write");
        stack.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
