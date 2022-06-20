fn main() {
    //プリミティブ型
    let ii = 1_i8;
    let jj = 2_u16;
    let kk: i32 = ii as i32 + jj as i32; // as 型名でキャストできる

    println!("kk = {}", kk);

    //配列
    //固定長、ループ時にはリファレンス渡す
    let ary = [0, 1, 2, 3]; //[i32; 4]型

    for elem in &ary {
        println!("{}", elem);
    }
    println!("ary[1] = {}", ary[1]);
    
    //タプル
    //異なる型の値を格納可だが、イテレーターで回せない
    let tup = (0, "test", true);
    println!("{},{},{}",tup.0,tup.1,tup.2);

    //スライス
    //既存要素のリファレンス
    let ary_sliced = &ary[0..2];
    for elem in ary_sliced {
        println!("{}", elem);
    }

    //ベクター
    //可変長＆
    let mut v= vec![0, 1, 2, 3];
    
    println!("beforepush:{:?}", v);

    v.push(10);
    println!("afterpush:{:?}", v);

    //内部要素を変更
    v[2] += 1;
    println!("afterv[2]+=10:{:?}",v);
    //スライスで部分列を取り出すことも可能
    println!("&v[3..]={:?}",&v[3..]);

}
