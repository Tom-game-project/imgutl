#[warn(unused_imports)]
use std::fs::File;
use std::io::{Read, Write};

fn isPNG(image_data:&Vec<u8>)->bool{
    let png_sig:[u8;8]=[137, 80, 78, 71, 13, 10, 26, 10];
    for (i,j) in png_sig.iter().enumerate(){
        if image_data[i]!=*j{
            return false
        }
    }
    return true
}

fn main() {
    // ファイルをバイナリモードで開く
    let mut input_file = File::open("sub\\img.png").expect("Failed to open input file");

    // ファイルのデータを読み込む
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer).expect("Failed to read input file");
    println!("{}",isPNG(&buffer));
    println!("{:?}",buffer);
    println!("File manipulation completed.");
}
