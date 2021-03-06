mod vec;

use vec::Vec2;
use vec::Vec3;
use vec::IpAddrKind;

fn main() {
    let x:u8 = 6;
    let mut x = x * 2;
    x = x+4;
    let x = x / 3;
    //const x:u8 = x + 34; //letは実行時に値が決まる(あくまで不変な変数)　constは値がコンパイル時に決まってないといけないのでダメ
    //x = 0;
    let y:u8 = 30;
    //const y:u8=50;//変数と定数でシャドイングを使うことは不可
    println!("Hello, world!");
    func(x, y);

    fun_tpl2((32,256,65539));

    show_char_code('0');
    
    let num = 4;
    println!("{} is even : {}", num, is_even(num));

    looping();

    recur(0);

    is3n(5);

    let num = 7;
    println!("is5n({})={}",num,is5n(num));
    let num = Some(8);
    println!("is5n({:?})={}",num,is5n_some(num));

    let x = 30;
    let y = 4;
    let mut v = Vec2{
        x, //初期化に使う変数名 = フィールド名の時に許される省略記法
        y,
    };
    let v2 = Vec2{
        y : 40,
        ..v //更新記法で残りのフィールドをほかのインスタンスと同じ値にセットできる
    };
    v.x = 3;
    println!("{:?}.length={}", v,v.length());
    println!("{:?} + {:?} = {:?}", v,v2,v.add(&v2));
    println!("{:?} + {:?} = {:?}", v,v2,Vec2::add2(&v, &v2));

    let v3 = Vec3(3, 4, 5); //タプル構造体はタプルと同じように初期化・使用
    println!("{},{},{}", v3.0,v3.1,v3.2);

    let ip4 = IpAddrKind::V4(192,2,0,0);
    let ip6 = IpAddrKind::V6(String::from("::1"));
    ip4.print_addr();
    ip6.print_addr();

    print_string();
}

fn print_string(){
    //文字列リテラルとString型
    //str,str2とも可変化可能
    let str = "hello.";
    let mut str2 = String::from("hello..."); //この時点でメモリ確保が行われている。String str = new String("...");相当
    //str.push_str("world."); //実はString型じゃない
    str2.push_str("world.");
    
    println!("{:?}, {:?}",str,str2);

    let str3 = String::from("yellow.");
    //let str4 = str3; //こうすると、str3はもう有効な変数でないと判定される。以降は使えない
    //スタックに積む型に対してはCopyトレイトを実装できて、暗黙のうちに深いコピーを作れる
    //言い換えると深いコピーも浅いコピーも違いがないようなもの
    //Dropトレイトを実装しているような型にはCopyトレイトを実装できない(コンパイラがチェックする)
    //明示的に深いコピーが欲しければcloneする
    let mut str4 = str3.clone();

    str4.push_str("red.");

    println!("{},{}", str3, str4);

    let str5 = String::from("apple");
    let some_num = 10;
    
    func_str(str5, some_num);
    //StringはDrop実装なので関数に渡しても所有権が移動する
    //u32はCopyトレイト実装なので、所有権は移動しない

    let str6 = String::from("lemon");
    let str7 = id_str(str6);//str6がid_str関数にムーブ→戻り値が呼び出し元にムーブ
    let str8 = create_str();
    //println!("{}",str5); //エラー。無効な変数を使った
    println!("{}",str7); //関数によって移動された所有権を使う
    println!("{}",str8); //関数によって移動された所有権を使う
    println!("{}",some_num); //所有権は移動しないので有効なまま

    println!("Length of {} is {}", str7, str_length(&str7)); //参照渡し。Readonly

    let mut str9 = String::from("world");
    add_hello(&mut str9);
    add_hello(&mut str9);
    add_hello(&mut str9);//add_helloを何回呼んでも、str9の可変参照を利用するのは常に1つだけ
    println!("{}",str9);

    let r_str9_1 = &mut str9;
    //let r_str9_2 = &mut str9;//2つの可変参照が同時に使われるとエラー
    //同時に可変・不変両方で参照もできない
    println!("{},", r_str9_1);

    let mut str10 = String::from("grape");
    {//r_str10_1はこのカッコのスコープしかない
        let r_str10_1 = &mut str10;
        add_hello(r_str10_1);
        r_str10_1.push_str("tree");
        println!("{}",r_str10_1);
    }
    let r_str10_2 = &mut str10;
    r_str10_2.push_str("house");
    println!("{}",r_str10_2);

}//スコープを抜けたら、確保されたメモリが直ちに解放される。drop関数が自動で呼ばれる。free();に相当
//C++でRAIIパターンとして結構有名

fn func_str(s : String, x : u32){
    //所有権が移った側で使うならよい
    println!("{}", s);
}//sはdropされる

fn id_str(s : String) -> String{
    s
}//返り値なので呼び出し元へムーブされる

//文字列の長さを返す関数
//Stringオブジェクトへの参照を取る(借用する)ので所有権を奪わない
fn str_length(s : &String) -> usize {
    s.len()
}//sは参照なので所有権持ってない　dropは呼ばれない

fn add_hello(s : &mut String){
    s.push_str("hello");
}

fn create_str() -> String{
    String::from("yes/no")
}//返り値なので呼び出し元へムーブされる

//fn create_str2() -> &String{
fn create_str2() -> String{
    let s = String::from("yes/no/cancel");
    //&s //参照を返した後でsがdropされてしまう。参照は必ず有効な変数でないといけない
    s      //参照ではなく所有権ごと返す。
    //あるいはライフタイム指定子を使う
}

fn func(x : u8, y : u8){
    println!("x={}, y={}",x,y);
}
fn show_char_code(c : char){
    //let code = c as u32;
    /*let code = {
        let code_tmp = c;
        code_tmp as u32
    };*/
    let code = get_char_code(c);

    let x = {
        let ss = 3;
        println!("{ss}");
    };
    //println!("char code of '{}' is {}(0x{:x})", c,code,code);
    println!("char code of '{}' is {code}(0x{code:x})", c);
}
fn get_char_code(c : char) -> u32 {
    c as u32 //return c as u32;と同義
}
fn is_even(x : u32) -> bool{
    let rem = x % 2;
    //let result = false;
    //let mut result = false;
    // if rem == 0{
    //     /*let*/ result = true; //resultをif式の中でシャドイングしてもスコープがブロック内でしか有効でないからダメ
    // } else {
    //     /*let*/ result = false;
    // }

    //if"式"なのでこれでいい
    let result = if rem == 0 {
        true
    } else {
        false
    };
    result
}
fn fun_tpl(x : (u8, u16, u32)){
    println!("({},{},{})",x.0, x.1, x.2);
}
fn fun_tpl2((x,y,z) : (u8, u16, u32)){
    println!("({},{},{})",x,y,z);
}
fn looping(){
    let mut n = 0;
    while n < 10 {
        println!("{}...",n);
        //n++; //Rustには無い
        n += 1;
    }

    //これでwhile(true){処理}と同義
    //loop() {
    //
    //}

    //for文(=foreach)のほうが優秀
    //0..10はRange型インスタンス、メソッド呼びたい場合は()つける
    for iter in 0..10 {
        println!("{}...",iter);
    }
}
fn recur(x : u8){
    if x < 10{
        println!("{}...", x);
        recur(x+1);
    }
}
fn is3n(x : u32){
    match x%3 {
        0 => {println!("0");},
        1 => {println!("1");},
        2 => {println!("2");},
        _ => (),
    }
}
fn is5n(x : u32) -> bool{
    // if(x % 5 == 0){
    //     true
    // } else {
    //     false
    // }

    // match x % 5{
    //     0 => true,
    //     _ => false,
    // }

    //ただの数値型だと恩恵あんまりないかも
    if let 0 = x % 5{
        true
    } else {
        false
    }
}

fn is5n_some(x : Option<u32>) -> bool{
    // if(x.expect("") % 5 == 0){
    //     true
    // } else {
    //     false
    // }

    // match x{
    //     Some(n) => n % 5 == 0,
    //     _ => false,
    // }

    //xそのものが何かと一致したときだけ処理をしたい場合にif letが有効
    //種類の多いenum型のときに効果を発揮する
    if let Some(n) = x { 
    //if let x = Some(n) { //これはエラー　順番気を付ける
        n % 5 == 0
    } else { //このelseを書かないとエラー
        false
    }

    // if let 0 = x.expect("") % 5 { //いまいちコード 今回の場合はmatchが一番いい気がする
    //     true
    // } else {
    //     false   
    // }
    
}
