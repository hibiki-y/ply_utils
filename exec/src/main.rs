extern crate decoder;
fn main() {
    println!("cut start");
    decoder::to_string(
        "./original/dancer_vox11_00000001.ply",
        "./out/decode_test11.ply",
    );
}
