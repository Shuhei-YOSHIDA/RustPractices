/// # practice1
/// sandbox of language


// ライフタイム
fn bar<'a> (x: &'a i32) {

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

}
