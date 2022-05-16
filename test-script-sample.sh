#! /bin/bash
unalias -a

# note 執筆用に作成したテスト
# やっている内容は tests/cli.rs > test_stdinout と同じ

cargo build
mkdir -p ./tests/temporary
input=./tests/input/testin.csv # 事前に準備したインプット
expected=./tests/expected/testout.txt # 事前に準備した結果(期待)
result=./tests/temporary/output.txt # 実際の実行結果を保存するファイル

# ビルドしたプログラムを使って処理を行った結果をファイルに保存
<$input ./target/debug/csvprint.exe >$result

echo "期待値 '$expected' と実行結果 '$result' を比較します。"
if diff $expected $result
then
    echo "OK"
else
    echo "FAIL"
fi
