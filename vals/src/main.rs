fn main() {
    let x = "Hello".to_string();
    let y = x;
    // println!("{}", x); // 所有権エラー
    println!("{}", y);

    let y2 = y.clone(); // エラー回避方法①　あらかじめコピーしておく

    myprint(y); //yの所有権がmyprint()へ移動する

    //yの所有権がないため初期化されてない変数としてエラーになる
    // myprint(y);

    myprint(&y2);// エラー回避方法②　あらかじめコピーしておく
    println!("{}", y2);


    let mut s = "Hello!".to_string();
    println!("s={}", s);


    // myclear(&s); //関数に渡したリファレンス（＆s）が変更される場合はミュータブルなリファレンスを渡す(&mut hoge) 
    let s_ref = &mut s;
    myclear(s_ref);
    println!("s={}", s);

    // let h = return_hi(); //関数内のローカル変数へのリファレンスを返却値にすることはできない
    // println!("{}", h);
    
}

fn myprint<T: std::fmt::Display>(msg: T) {
    println!("in myprint: {}", msg);
}

fn myclear(x: &mut String){
    x.clear();
}


fn return_hi() -> &String {
    let s = "Hi!".to_string();
    &s
}