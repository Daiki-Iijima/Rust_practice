# Cargoとは？
- Rustのビルドシステム兼、パッケージマネージャ
- Rustのプロジェクト管理するのに便利
- バージョン管理システムも自動でいれてくれる(Git)

# インストール
- Rustのインストーラーでインストールしていればすでに入っている
- インストールされているか確認
```bash
cargo --version
=>cargo 1.38.0 (23ef9a4ef 2019-08-20)
```

# プロジェクトの作成方法
1. ターミナルから下記コマンドを実行
```bash
cargo new プロジェクト名 --bin
```
- 「--bin」 : バイナリを作成

2. こんなの出ればOK
```bash
Created binary (application) `hello_cargo` package
```

# 生成時のフォルダ構造
いつか書く

# フォルダ内に生成される「.toml」ファイルについて
- Tom's Obvious,Minimal Language : (直訳) トムの明確な最小限の言語
- Cargoの設定フォーマット

# Cargoのビルド、実行
## ビルド
1. ビルドしたいcargoプロジェクトのディレクトリに移動

2. ターミナルから
```bash
cargo build
=>
Compiling hello_cargo v0.1.0 (/Users/iijimadaiki/Desktop/Git_ws/Rust_practice/hello_world/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 4.34s
```
3. target/debug/ディレクトリに「hello_cargo」が生成される

## 実行
```bash
./target/debug/hello_cargo
```

## buildと実行を一つのコマンドで行う
```bash
cargo run
```

# buildできるか確認する
- 使用するとbuildするよりも早くコンパイル可能(ミスがない)かを確認できる
- 実行可能ファイルを**生成しない**

```bash
cargo check
```

# リリースビルドを行う
## リリースビルドを行うメリット・デメリット
### メリット
- 普通のビルドより最適化されてRustのコードを早くしてくれる
### デメリット
- ビルドをする時間が長くなる(最適化してるから)
## ビルド
```bash 
cargo build --release
```
## 実行
```bash
./target/release/hello_cargo
```

# cargoをbuildすると生成されるCargo.lockファイルとは？
そのプロジェクトの依存の正常なバージョンを追いかけるためのファイル

**手動で変更する必要はない**


# cargoのビルドはインクリメントビルド
- ファイルに変更があるかを確認して、コンパイルしてる