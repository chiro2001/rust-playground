/// `test` begin
/// 
/// # Test life time:
/// 
/// ```rust,should_panic
/// let r;
///   {
///     let x = 1;
///     r = &x;
///   }
///   println!("r = {r}");
/// ```


fn longest<'b>(x: &'b str, y: &'b str) -> &'b str {
  if x.len() > y.len() {
      x
  } else {
      y
  }
}

fn main() {
  let r;
  {
    let x = 1;
    r = x;
  }
  println!("r = {r}");
  let x = "aa";
  let y = "bbb";
  println!("{}", longest(x, y));
}