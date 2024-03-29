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
  fn summarize(&self) -> String {
    String::from("(read more...)")
  }
}
trait Message {
  fn message(&self) -> String {
    String::from("Message")
  }
}

struct NewsArticle {
  headline: String,
  location: String,
  author: String,
  content: String,
}
impl Summary for NewsArticle {
  // fn summarize(&self) -> String {
  //   // format! : 埋め込んだ値をStringで返してくれる
  //   format!("{}, by {} ({})", self.headline, self.author, self.location)
  // }
}
impl Message for NewsArticle {}

struct Tweet {
  username: String,
  content: String,
  reply: bool,
  retweet: bool,
}
impl Summary for Tweet {
  fn summarize(&self) -> String {
    // format! : 埋め込んだ値をStringで返してくれる
    format!("{}: {}", self.username, self.content)
  }
}

pub fn run() {
  let apple = Apple {};
  let banana = Banana {};
  get_price(apple);
  get_price(banana);

  let tweet = Tweet {
    username: String::from("test"),
    content: String::from("test"),
    reply: false,
    retweet: false,
  };
  println!("tweet: {}", tweet.summarize());

  let article = NewsArticle {
    headline: String::from("test"),
    location: String::from("test"),
    author: String::from("test"),
    content: String::from(
      "test \
       test",
    ),
  };
  println!("article: {}", article.summarize());

  notify(&article);
  notify_another(&article); // &tweetは渡せない
}

// priceを出力する
fn get_price<T: Fruits>(fruits: T) {
  println!("price: {}", fruits.price());
}

fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}

fn notify_another(item: &(impl Summary + Message)) {
  println!("Breaking news! {}", item.summarize());
  println!("Message: {}", item.message());
}
