import sys

total = 0
# コマンドライン引数を順に足す
for i, v in enumerate(sys.argv):
    if i == 0:
        # 0番目はコマンド自身なので飛ばす
        continue
    try:
        # 文字列を数値に変換
        total += float(v)
    except ValueError:
        pass
# 結果表示
print(total)
