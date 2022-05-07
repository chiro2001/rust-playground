use std::collections::HashMap;

fn main() {
  let mut map = HashMap::new();
  map.insert("t1", "test");
  map.insert("t2", "test test");
  map.insert("t3", "test test test");
  map.keys().for_each(|key| println!("map({:?}) = {:?}", key, map.get(key)));
  map.keys().for_each(|&key| println!("map({:?}) = {:?}", key, map.get(key)));
  let keys_map: Vec<_> = map.keys().map(|key| String::from(*key) + "-1").collect();
  println!("{:?}", keys_map);
  map.keys().for_each(|key| println!("map({:?}) = {:?}", key, map.get(key)));
  map.keys().for_each(|&key| println!("map({:?}) = {:?}", key, map.get(key)));
  let test_key = "ss";
  match map.get(&test_key) {
    Some(value) => println!("map(ss) = {value}"),
    None => println!("{test_key} not found in map!")
  };
  let test_key2 = "t4";
  println!("t4 = {}", map.entry(test_key2).or_insert(&"test test test test"));
  println!("{:?}", map);
  for (key, value) in &map {
    println!("for: map{key} = {value}");
  }
  println!("{:?}", map);
  println!("{:?}", map.get("ssss"));
}