// ビルドするためにgccのインストールが必要
fn main() {
    // printlnは最後に開業を含める
    println!("Hello, world!");

    // print!は最後に改行を含めない
    print!("こ");
    print!("ん");
    print!("に");
    print!("ち");
    print!("は");

    // eprintは標準エラー出力
    eprint!("エラーです");
    eprintln!("エラーで");
    eprintln!("す");

    // {}に整数リテラル2を埋め込む
    // {}はプレースホルダーと呼ばれる
    println!("最小の素数は{}です．", 2);

    // 計算
    // 他の言語と同じ
    println!("2 + 3 = {}", 2 + 3);
}
