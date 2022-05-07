fn is_prime(n: u64) -> bool {
    if n <= 2 && n > 0 {
        true
    } else {
        let mut flag = true;
        for i in 2..=((n as f64).sqrt() as u64) {
			if !flag {
                break;
            }
            flag = n % i != 0;
        }
        flag
    }
}

fn main() {
	for i in 1..100 {
		println!("{i} {text} a prime.", i=i, text=match is_prime(i) {
			true => "is",
			_ => "is not"
		})
	}
}
