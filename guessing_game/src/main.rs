use std::io;

fn main() {
    println!("数をあててごらん");

    println!("予測値を入力してくれ");

    //  let : immutable(不変)変数の定義
    //  mut : mutable(可変)変数の定義に変更
    let mut guess = String::new();

    //  stdin() : ターミナルへの標準出力
    //  read_lines : 標準入力ハンドル
    //  (&mut guess) : 参照型のmutable(可変変数)guessの定義
    //      - &mutなのはユーザーから入力された値を受け取る引数は、可変である必要があるから
    io::stdin().read_line(&mut guess)
    .expect("読めないお");

    println!("お主の予想 : {}",guess);
}
