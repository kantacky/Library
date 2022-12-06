//
// (C) 2022 Kanta Oikawa
//
// Prime - prime_factorize.rs
//

use std::io;

fn prime_factorize(mut n: usize) -> Vec<[usize; 2]> {
  let mut res: Vec<[usize; 2]> = Vec::new();

  let mut p: usize = 1;
  while p * p < n {
    p += 1;

    if n % p != 0 {
      continue;
    }

    let mut e: usize = 0;
    while n % p ==0 {
      e += 1;
      n /= p;
    }

    res.push([p, e]);
  }

  if n != 1 {
    res.push([n, 1]);
  }

  return res;
}

fn main() {
  let mut s: String = String::new();
  io::stdin().read_line(&mut s).ok();
  let n: usize = s.trim().parse().ok().unwrap();

  let pf = prime_factorize(n);

  print!("{} = ", n);
  for f in &pf {
    if f[1] == 1_usize {
      print!("{} ", f[0]);
    } else {
      print!("{}^{} ", f[0], f[1]);
    }

    if f != &pf[pf.len() - 1] {
      print!("* ");
    }
  }
  print!("\n");
}
