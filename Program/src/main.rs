use std::{i32, io::{self, Write}}; // Disini pake {self, Write} biar bisa pake flush unwrap di baris 5, 9, 13

fn main() {

    let mut kumpulan_angka: Vec<i32> = Vec::new();
    let mut min:i32 = i32::MAX;
    let mut max:i32 = i32::MIN;

    let mut input_boundary = String::new();
    print!("Masukkan jumlah angka yg ingin diinput\t: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_boundary).expect("Gagal input");
    let boundary:i32 = input_boundary.trim().parse().expect("Masukkan angka!");

    for i in 0..boundary {
        print!("Masukkan nilai ke-{}\t: ", i+1);
        io::stdout().flush().unwrap();
        
        // Siapin string untuk tampung input dari terminal
        let mut input_nilai = String::new();
        io::stdin().read_line(&mut input_nilai).expect("Gagal baca input");
        let angka:i32 = input_nilai.trim().parse().expect("Masukkan angka yang valid (-128 sampai 127)!");

        if angka < min {
            min = angka;
        }
        if angka > max {
            max = angka;
        }
        
        kumpulan_angka.push(angka);
    }

    print!("Angka yang kamu ketik: ");
    for angka in &kumpulan_angka {
        print!("{} ", angka);
    }
    println!();

    if !kumpulan_angka.is_empty() {
        println!("Angka terbesar: {}", max);
        println!("Angka terkecil: {}", min);
    }
}