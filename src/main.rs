use std::io;

fn bilangan_armstrong(n: u32) -> bool {
    let digit: Vec<u32> = n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let pangkat = digit.len() as u32;
    let pertambahan_pangkat: u32 = digit.iter()
        .map(|&digit| digit.pow(pangkat))
        .sum();
    pertambahan_pangkat == n
}

fn main() {
    println!("Masukkan angka untuk mengecek apakah angka tersebut merupakan bilangan armstrong:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Masukkan Input Yang Benar bgst!");
    
    let input = input.trim(); 
    match input.parse::<u32>() {
        Ok(n) => {
            if bilangan_armstrong(n) {
                println!("{} merupakan bilangan armstrong.", n);
            } else {
                println!("{} bukan merupakan bilangan armstrong.", n);
            }
        }
        Err(_) => println!("Input tidak valid! Tolong masukkan bilangan bulat positif."),
    }
}

