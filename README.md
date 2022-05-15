# csvprint

Rust 学習中の練習用リポジトリ

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
