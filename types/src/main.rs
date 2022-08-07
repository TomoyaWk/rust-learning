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



    //文字型
    //char型　１文字のみ・シングルコーテーション
    let _c = 'c';

    //文字列型
    //ダブルコーテーション 静的領域に割り当てられる（メモリ上ではない）値のスライス
    //文字の追加操作などは不可
    let _s = "hello";

    //String型
    //
    let mut st = "Hello".to_string();
    let _st2 = String::from("Hello");
    st.push_str(", World");

    println!("{}", st);
    println!("{}",&st[0..6]);
    //タプル構造体
    struct RGB(u32, u32, u32);
    let color = RGB(255,128,0);
    println!("{},{},{}",color.0,color.1,color.2);
    //構造体について
    #[derive(Debug)]
    pub struct  Person {
        name: String,
        age: u8
    }
    //構造体にメソッド実装
    impl Person {
        //関連関数　構造体に関連する
        pub fn new(name: String, age: u8)->Person {
            Person { name, age}
        }
        //メソッド　引数として&self, selfを指定して処理対象インスタンスを指定する
        fn age_increment(&self, incr: u8)->u8 {
            self.age + incr
        }

        fn age_incre_replace(&mut self, incr: u8) {
            self.age += incr
        }
    }

    //各フィールドに値を設定（mutableにして変更することも可能）
    let jiro = Person{name: String::from("jiro"), age: 24};
    println!("{},{}",jiro.name, jiro.age);

    let mut taro=Person::new(String::from("taro"),10);
    let age_plus = taro.age_increment(1);
    println!("taro age = {}", age_plus);


    taro.age_incre_replace(15);
    println!("taro age = {}", &taro.age);
    //debugトレイトが実装されていると構造体（その他変数も）の中身を表示できる
    println!("{:?}", taro);

    
    #[derive(Debug)]
    struct Parents<'a, 'b> {
        father: &'a Person,
        mother: &'b Person
    }
    impl<'a, 'b> Parents<'a, 'b> {
        fn new(father: &'a Person, mother: &'b Person)-> Parents<'a, 'b> {
            Parents{father, mother}
        }
    }

    let saburo = Person{name:String::from("saburo"), age: 35};
    let hanako = Person{name:String::from("hanako"), age: 27};

    let sato_family = Parents::new(&saburo, &hanako);

    println!("{:?}", sato_family);


    //列挙型
    enum Sign {
        Positive,
        Zero,
        Negative
    }

    fn determine_sign(x: i32) -> Sign {
        if x < 0 {
            Sign::Negative
        } else if x > 0  {
            Sign::Positive
        } else {
            Sign::Zero
        }
    }

    fn print_sign (s: Sign) {
        match s {
            Sign::Positive => println!("+(Positive)"),
            Sign::Zero => println!("0(Zero)"),
            Sign::Negative => println!("-(Negative)")
        }
    }

    print_sign(determine_sign(1));
    print_sign(determine_sign(0));
    print_sign(determine_sign(-1));

}
