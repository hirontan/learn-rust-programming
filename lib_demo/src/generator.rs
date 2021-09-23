// https://crates.io/crates/rand
use rand::Rng;

// ランダムの整数を生成
pub fn get_ran() -> u8 {
  let mut rng = rand::thread_rng();
  let n: u8 = rng.gen();
  n
}
