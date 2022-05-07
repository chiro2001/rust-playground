fn main() {
    dbg!((-64..64)
        .map(|i| i * i)
        .filter(|&i| i % 2 == 0)
        .fold(0, |s, i| s + i));
}
