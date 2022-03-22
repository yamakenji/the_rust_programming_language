fn main() {
    // println!("Hello, world!");

    // 新しいVectorを生成する
    let v: Vec<i32> = Vec::new();
    // vec!マクロを使って値を保持するVectorを生成する
    let v = vec![1, 2, 3];

    // Vectorを更新する

    // ベクタを生成し、それから要素を追加する
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Vectorの要素を読む
    // 要素を得る2つの方法とは、&と[]を使用して参照を得るものと、
    // getメソッドに引数として添え字を渡してOption<&T>を得るものだということです。
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // ベクタに含まれない要素の添え字を使おうとしたときのプログラムの振る舞い
    // プログラムをパニックさせる
    // let does_not_exist = &v[100];
    // getメソッドにベクタ外の添え字を渡すと、パニックすることなくNoneを返す
    // let does_not_exist = v.get(100);

    // ベクタの最初の要素への不変参照を保持しつつ、終端に要素を追加しようとしています。
    // 関数内のここ以降で、この要素（訳注：firstのこと）を参照しようとすると失敗します。
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // v.push(6);
    println!("The first element is: {}", first);

    // ベクタ内の値を順に処理する
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        println!("{:?}", i)
    }
}
