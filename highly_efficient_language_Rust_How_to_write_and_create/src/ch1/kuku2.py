# Pythonで九九の表（行末のカンマを削ったもの）
for y in range(1, 10):
    a = [f"{x * y:3}" for x in range(1, 10)]
    print(",".join(a))
