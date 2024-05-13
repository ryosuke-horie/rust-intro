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
}
