pub fn bilangan_armstrong(n: u32) -> bool {
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

