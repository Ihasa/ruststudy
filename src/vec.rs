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
