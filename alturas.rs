fn main() {
  let mut alt_francisco = 150;
  let mut alt_sara = 110;
  let mut idd = 0;
  while alt_sara <= alt_francisco{
    alt_francisco += 2;
    alt_sara += 3;
    idd += 1;
    println!("Francisco tem {} e Sara tem {}", alt_francisco, alt_sara)
  };
  println!("Anos: {}", idd);
}
