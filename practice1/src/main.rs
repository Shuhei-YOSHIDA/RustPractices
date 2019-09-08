/// # practice1
/// sandbox of language


// ライフタイム
fn bar<'a> (x: &'a i32) {

}

// generics, traits
fn hoge<T: std::fmt::Display+std::f64::sin>(x: T) {
    println!("{}", x.sin());
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
}
