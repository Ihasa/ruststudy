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
}

fn func(x : u8, y : u8){
    println!("x={}, y={}",x,y);
}

fn fun_tpl(x : (u8, u16, u32)){
    println!("({},{},{})",x.0, x.1, x.2);
}
fn fun_tpl2((x,y,z) : (u8, u16, u32)){
    println!("({},{},{})",x,y,z);
}
