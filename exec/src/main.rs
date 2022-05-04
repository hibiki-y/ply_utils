extern crate cutter;
extern crate decoder;
fn main() {
    println!("cut start");
    cutter::cut_property("../test/test.ply", "../out/output_test-1.ply", 0, 3);
    println!("decode start");
    decoder::to_string(
        "../original/dancer_vox11_00000001.ply",
        "../out/decode_test11.ply",
    );
}
