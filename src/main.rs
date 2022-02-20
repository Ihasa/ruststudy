mod vec;

use vec::Vec2;

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
    v.x = 32;
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
