fn main() {
    dbg!((-999999..0)
        .map(|i| (i as i128 * i as i128) as i128)
        .filter(|&i| i % 2 == 0)
        .fold(0, |s, i| s + i));
}
