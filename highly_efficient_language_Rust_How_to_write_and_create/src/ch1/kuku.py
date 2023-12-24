# Pythonで九九の表を作成
for y in range(1, 10):
    for x in range(1, 10):
        print(f"{y * x:3},", end="")
    print("")
