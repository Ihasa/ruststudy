#[derive(Debug)]    //トレイトの導出 ほかの言語でクラスの継承って呼ぶはずのもの
                    //Debugトレイトによってprintln!マクロでデバッグ用出力ができる
pub struct Vec2{
    pub x : u32,
    pub y : u32,
}

impl Vec2 {
    pub fn length(&self) -> f64{ //第1引数は必ずself 所有権ほしければmutにしてもいい 今回は参照だけなので不変
        ((self.x * self.x + self.y * self.y) as f64).sqrt()
    }
    pub fn add(&self,op : &Vec2) -> Vec2{
        Vec2{
            x : self.x + op.x,
            y : self.y + op.y,
        }
    }
    pub fn add2(op1 : &Vec2, op2 : &Vec2) -> Vec2{ //第1引数をselfにしない場合は関連関数扱い(=staticメソッド)
        Vec2{
            x : op1.x + op2.x,
            y : op1.y + op2.y,
        }
    }
}

pub struct Vec3(pub u32,pub u32,pub u32); //タプル構造体 フィールド名つけるまでもないような場合に使う タプルと同じ使い方が可能

#[derive(Debug)]
pub enum IpAddrKind{
    V4(u8, u8, u8, u8), //enumといいつつデータを持てる
    V6(String)
}

impl IpAddrKind{        //enumといいつつメソッド定義できる
    pub fn print_addr(&self){
        println!("{:?}",self);
        match(self){
            IpAddrKind::V4(n1,n2,n3,n4) => println!("{}.{}.{}.{}", n1,n2,n3,n4),
            IpAddrKind::V6(s) => println!("{}",s),
        }
    }
}

enum Vec2_3{        //こうすればInterface的に使えそう
    V2(Vec2),
    V3(Vec3)
}
