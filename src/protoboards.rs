use std::fs::File;
use std::io::{BufWriter, Write};

pub fn generate_protoboards() {
    type ProtoBoard = u16;

    let file = File::create("protoboards.txt").expect("Unable to create file");
    let mut writer = BufWriter::new(file);

    let mut total_count = 0;

    for t in 2..=16 {
        let mut boards_for_t = Vec::new();

        for x in 0u16..=u16::MAX {
            if x.count_ones() == t {
                boards_for_t.push(x);
            }
        }

        writeln!(writer, "=============================").expect("Unable to write");
        writeln!(writer, "  Boards with t = {} filled tiles", t).expect("Unable to write");
        writeln!(writer, "=============================\n").expect("Unable to write");

        println!("t = {}: {} boards", t, boards_for_t.len());

        for (index, board) in boards_for_t.iter().enumerate() {
            total_count += 1;

            writeln!(writer, "Board #{} (t = {} filled tiles):", total_count, t)
                .expect("Unable to write to file");

            for row in 0..4 {
                for col in 0..4 {
                    let bit_index = row * 4 + col;
                    let cell = (board >> bit_index) & 1;
                    let symbol = if cell == 1 { 'X' } else { '.' };
                    write!(writer, "{} ", symbol).expect("Unable to write to file");
                }
                writeln!(writer).expect("Unable to write to file");
            }

            writeln!(writer).expect("Unable to write to file"); // extra newline between boards
        }
    }

    println!("All {} protoboards written to protoboards.txt", total_count);
}
