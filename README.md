# Rustの環境構築

### rustupのインストール

公式
https://www.rust-lang.org/ja/tools/install

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

* 最新版のインストール、及びバージョンアップ

```bash
$ rustup update
```

* バージョンの確認

```bash
$ rustup --version
$ rustc --version
$ cargo --version
```

### Cargo

公式
https://doc.rust-lang.org/cargo/getting-started/first-steps.html

* プロジェクトの作成

```bash
$ cargo new hello_world
$ cd hello_world
```

* ビルド

```bash
$ cargo build
```

* ビルドと実行

```bash
$ cargo run
```

### Createの追加

* cargo add

```bash
$ cargo add ferris-says
```

* CreateをCargo.tomlの[dependencies]に追記

```Cargo.toml
[dependencies]
ferris-says = "0.2"
```
