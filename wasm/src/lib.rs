use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);
}

#[wasm_bindgen]
pub fn is_prime(num: String) -> bool {
  if let Ok(num) = num.parse() {
    return is_prime_u8(num);
  }

  if let Ok(num) = num.parse() {
    return is_prime_u16(num);
  }

  if let Ok(num) = num.parse() {
    return is_prime_u32(num);
  }

  if let Ok(num) = num.parse() {
    return is_prime_u64(num);
  }

  if let Ok(num) = num.parse() {
    return is_prime_u128(num);
  }

  false
}

fn is_prime_u8(num: u8) -> bool {
  if num == 0 || num == 1 {
    return false;
  }

  let mut i = 2;

  while i * i <= num {
    if num % i == 0 {
      return false;
    }

    i += 1;
  }

  true
}

fn is_prime_u16(num: u16) -> bool {
  if num == 0 || num == 1 {
    return false;
  }

  let mut i = 2;

  while i * i <= num {
    if num % i == 0 {
      return false;
    }

    i += 1;
  }

  true
}

fn is_prime_u32(num: u32) -> bool {
  if num == 0 || num == 1 {
    return false;
  }

  let mut i = 2;

  while i * i <= num {
    if num % i == 0 {
      return false;
    }

    i += 1;
  }

  true
}

fn is_prime_u64(num: u64) -> bool {
  if num == 0 || num == 1 {
    return false;
  }

  let mut i = 2;

  while i * i <= num {
    if num % i == 0 {
      return false;
    }

    i += 1;
  }

  true
}

fn is_prime_u128(num: u128) -> bool {
  if num == 0 || num == 1 {
    return false;
  }

  let mut i = 2;

  while i * i <= num {
    if num % i == 0 {
      return false;
    }

    i += 1;
  }

  true
}
