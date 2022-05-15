# csvprint

**Rust 学習中の練習用リポジトリ**

学習のために作成してるコードになりますので、クローンして使用する際は自己責任でお願い致します。

## 概要

ヘッダーありCSVを標準入力から読んで、
標準出力へ次のフォーマットで表示する。

```txt
<!-- RECORD BEGIN (record number) -->
<!-- COLUMN BEGIN (column name) -->
<!-- COLUMN END   (column name) -->
...
<!-- RECORD END   (record number) -->
...
```

## コマンドライン

```sh
<対象の.csv csvprint(.exe)
```

## 例

元のCSVが下記だとすると…
```csv
name,age
"John ""BE"" Constantine",39
"MY NAME IS
<b>PRINCE</b>
AND I'M FUNKY",1042
```
次のように出力
↓
```txt
<!-- RECORD BEGIN 1 -->
<!-- COLUMN BEGIN name -->
John "BE" Constantine
<!-- COLUMN END   name -->
<!-- COLUMN BEGIN age -->
39
<!-- COLUMN END   age -->
<!-- RECORD END   1 -->
<!-- RECORD BEGIN 2 -->
<!-- COLUMN BEGIN name -->
MY NAME IS
<b>PRINCE</b>
AND I'M FUNKY
<!-- COLUMN END   name -->
<!-- COLUMN BEGIN age -->
1042
<!-- COLUMN END   age -->
<!-- RECORD END   2 -->
```

## 参考

### testの書き方。main.rc, lib.rc をわける

Command-Line Rust
https://amzn.to/38msbql

### csv

【翻訳】RustとCSV解析
https://qiita.com/algebroid/items/c456d4ec555ae04c7f92

Crate csv
https://docs.rs/csv/latest/csv/

### io

RustのファイルI/OにはBufReader, BufWriterを使いましょう、という話
https://qiita.com/gyu-don/items/50f4239fc856bed00dc4

Rustで高速な標準出力
https://keens.github.io/blog/2017/10/05/rustdekousokunahyoujunshutsuryoku/
