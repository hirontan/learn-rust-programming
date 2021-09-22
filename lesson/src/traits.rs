// Trait: 複数の型のタイプを共通する機能を持たせたいときに利用する
trait Fruits {
  fn price(&self) -> u32;
}

struct Apple;
impl Fruits for Apple {
  fn price(&self) -> u32 {
    10
  }
}

struct Banana;
impl Fruits for Banana {
  fn price(&self) -> u32 {
    5
  }
}

pub fn run() {
  let apple = Apple {};
  let banana = Banana {};
}

// priceを出力する
fn get_price<T: Fruits>(fruits: T) {
  println!("price: {}", fruits.price());
}
