fn main() {
    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    println!("{}", v[0]);

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("{}", v[0]);

    {
        let v = vec![1, 2, 3, 4];
        println!("{}", v[0]);
    }

    // `let v = vec![1, 2, 3, 4];`スコープから外れたので、結果が`5`になる
    println!("{}", v[0]);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("{:?}", third);
    let third: Option<&i32> = v.get(2);
    // println!("{:?}", third);

    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);

    let mut v = vec![1, 2, 3, 4, 5];
    
    let first = &v[0];
    
    v.push(6);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }


    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let mut s = String::new();

    let data = "initial contents";
    println!("{}", data);
    
    let s = data.to_string();
    println!("{}", s);
    
    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    
    let s = String::from("initial contents");
    println!("{}", s);

    let hello = String::from("السلام عليكم");
    println!("{}", hello);
    let hello = String::from("Dobrý den");
    println!("{}", hello);
    let hello = String::from("Hello");
    println!("{}", hello);
    let hello = String::from("שָׁלוֹם");
    println!("{}", hello);
    let hello = String::from("नमस्ते");
    println!("{}", hello);
    let hello = String::from("こんにちは");
    println!("{}", hello);
    let hello = String::from("안녕하세요");
    println!("{}", hello);
    let hello = String::from("你好");
    println!("{}", hello);
    let hello = String::from("Olá");
    println!("{}", hello);
    let hello = String::from("Здравствуйте");
    println!("{}", hello);
    let hello = String::from("Hola");
    println!("{}", hello);

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1はムーブされ、もう使用できないことに注意
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);
    
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);

    for c in "नमस्ते".chars(){
        println!("{}", c);
    }
    
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
