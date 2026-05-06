use std::mem;

fn main() {
    let xs = [1, 2, 3, 4, 5];

    let ys = [0; 500];

    println!("配列の最初の要素: {}", xs[0]);
    println!("配列の2番目の要素: {}", xs[1]);

    // 配列はスタック上に置かれる
    println!("配列 xs が {} バイト使用", mem::size_of_val(&xs));
    println!("配列 ys が {} バイト使用", mem::size_of_val(&ys));

    // 配列はスライスとして借用される
    analyze_slice(&xs);
    analyze_slice(&ys);
}

/// スライスを借用する
fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}
