//
// (C) 2022 Kanta Oikawa
//
// Quick - quick_sort.rs
//

use std::io;

pub fn quick_sort<T: PartialOrd + Clone>(source: &mut [T], left: usize, right: usize) {
  let pivot: T = source[(left + right) >> 1].clone();
  let mut pl: usize = left;
  let mut pr: usize = right;

  while pl <= pr {
    while source[pl] < pivot && pl < right {
      pl+= 1;
    }
    while source[pr] > pivot && pr > left {
      pr -= 1;
    }

    if pl <= pr {
      source.swap(pl, pr);

      if pr > 0 {
        pr -= 1;
      }
      pl += 1;
    }

    if left < pr {
      quick_sort(source, left, pr);
    }
    if right > pl {
      quick_sort(source, pl, right);
    }
  }
}

fn main() {
  println!("Quick Sort Test Program");

  let mut source_s: String = String::new();
  io::stdin().read_line(&mut source_s).ok();
  let mut source: Vec<usize> = source_s.split_whitespace().flat_map(|x| x.parse::<usize>()).collect();

  let size: usize = source.len() - 1;
  quick_sort(&mut source, 0, size);

  for e in source {
    print!("{} ", e);
  }
  print!("\n");
}
