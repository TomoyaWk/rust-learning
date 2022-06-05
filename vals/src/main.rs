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

}

fn myprint<T: std::fmt::Display>(msg: T) {
    println!("in myprint: {}", msg);
}
