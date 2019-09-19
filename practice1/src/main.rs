/// # practice1
/// sandbox of language


// ライフタイム
fn bar<'a> (x: &'a i32) {

}

// generics, traits
fn hoge<T: std::fmt::Display>(x: T) {
    //println!("{}", x.sin());
    println!("{}", x);
}

// Struct
struct Neko {
    a : String,
}

struct Foo {
    x : i32,
    y : Vec<f64>,
    z : String,
    //z : str, // str primitive is difficult for struct
    //n : Vec<Neko>, // How to use?
}

impl Foo {
    fn x(self) -> i32 {
        self.x
    }

    fn y(self) -> Vec<f64> {
        self.y
    }

    fn z(self) -> String {
        self.z
    }

    //fn n(self) -> Vec<Neko> {
    //    self.n
    //}
}

struct FooBuilder {
    x : i32,
    y : Vec<f64>,
    z : String,
    //z : str, // str primitive is difficult for struct
    //n : Vec<Neko>,
}

// not worked
impl FooBuilder {
    fn new() -> Self {
        Self {x : 0,
              y : vec![],
              z : "".to_string(),
              //n : vec![],
        }
    }

    fn x(&mut self, x:i32) -> &mut Self {
        self.x = x;
        self
    }

    fn y(&mut self, y:Vec<f64>) -> &mut Self {
        self.y = y;
        self
    }

    fn z(&mut self, z:String) -> &mut Self {
        self.z = z;
        self
    }

//    fn n(&mut self, n:Vec<Neko>) -> &mut Self {
//        self.n = n;
//        self
//    }

    fn spawn(self) -> Foo {
        Foo {
            x : self.x,
            y : self.y,
            z : self.z,
            //n : self.n,
        }
    }
}

fn main() {
    println!("Hello, world!");

    // 借用
    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
    }
    println!("{}", x);

    // ライフタイム

    // generics
    //let i : i32 = 1;
    let f : f64 = 10.;
    //hoge(i);
    hoge(f);

    // Struct, Builder pattern
    let mut neko = Neko {a: "nekko".to_string()};
    println!("{}", neko.a);
    neko.a = "innu".to_string();
    println!("{}", neko.a);

//    let neko1 = Neko {a: "NEKO1".to_string()};
//    let neko2 = Neko {a: "NEKO2".to_string()};
//    let foo = FooBuilder::new().x(10)
//                               .y(vec![111., 222., 333.])
//                               //.z("FOO".to_string())
//                               //.n(vec![Neko {a : "NEKO1".to_string()},
//                               //        Neko {a : "NEKO2".to_string()},])
//                               .spawn();
    //let x = foo.x();
    //let y = foo.y();
    //let z = foo.z();
    //let n = foo.n();

}
