# csvprint

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

例 :
```csv
name,age
"John ""BE"" Constantine",39
"MY NAME IS
<b>PRINCE</b>
AND I'M FUNKY",1042
```
↓
```txt
<!-- RECORD BEGIN 1 -->
<!-- COLUMN BEGIN name -->
John "BE" Constantine
<!-- COLUMN END   name -->
<!-- COLUMN BEGIN age -->
15
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
