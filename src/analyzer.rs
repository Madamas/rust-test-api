pub mod vector_analyzer {
  use std::collections::HashMap;

  pub fn stats(integers: Vec<u32>) {
    let mut new_vec: Vec<u32> = integers.clone();
    new_vec.sort_unstable();

    let mut mode: HashMap<u32,u32> = HashMap::new();
    let mut avg: f32 = 0.0;
    let mut mode_key: u32 = 0;
    let mut mode_value: u32 = 0;

    for num in new_vec.iter() {
      avg += (*num) as f32;
      mode.entry(*num)
        .and_modify(|e| { *e += 1; })
        .or_insert(1);
    }

    for (key, val) in mode.iter() {
      if *val > mode_value {
        mode_value = *val;
        mode_key = *key;
      }
    }
    avg = avg / (integers.len() as f32);

    let middle = new_vec.len() / 2;
    let median = new_vec[middle];

    println!("Incoming arr {:?}", integers);
    println!("Sorted arr {:?}\n", new_vec);
    println!("Average {:?}", avg);
    println!("Mode {:?}", mode_key);
    println!("Median {:?}", median);
  }
}