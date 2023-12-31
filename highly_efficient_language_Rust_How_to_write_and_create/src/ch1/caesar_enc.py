# pythonでシーザー暗号に変換する関数
def encrypt(text, shift):
    # 'A'と'Z'の文字コードを得る
    code_a = ord("A")
    code_z = ord("Z")
    # 結果を代入する変数を準備
    result = ""
    # 一文字づつ繰り返す
    for ch in text:
        # 文字コードに変換
        code = ord(ch)
        # AからZの間か？
        if code_a <= code <= code_z:
            # shift分だけ並びをずらす
            code = (code - code_a + shift) % 26 + code_a
        # 文字コードから文字に変換
        result += chr(code)
    return result


enc = encrypt("I LOVE YOU", 3)
dec = encrypt(enc, -3)
print(f"{enc} => {dec}")
