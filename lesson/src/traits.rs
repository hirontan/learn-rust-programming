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

trait Summary {
  fn summarize(&self) -> String;
}

struct NewsArticle {
  headline: String,
  location: String,
  author: String,
  content: String,
}
impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    // format! : 埋め込んだ値をStringで返してくれる
    format!("{}, by {} ({})", self.headline, self.author, self.location);
  }
}

struct Tweet {
  username: String,
  content: String,
  retry: bool,
  retweet: bool,
}
impl Summary for Tweet {
  fn summarize(&self) -> String {
    // format! : 埋め込んだ値をStringで返してくれる
    format!("{}: {}", self.username, self.content);
  }
}

pub fn run() {
  let apple = Apple {};
  let banana = Banana {};
  get_price(apple);
  get_price(banana);
}

// priceを出力する
fn get_price<T: Fruits>(fruits: T) {
  println!("price: {}", fruits.price());
}
