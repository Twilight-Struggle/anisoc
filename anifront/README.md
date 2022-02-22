# anifront

## setup

- eslint...もともと入っている
- prettier...https://dev-yakuza.posstree.com/react/prettier/

## backend とのコミュニケーション

### 送信

- make: POST のみ
- reset: id のみ
- mov: id と Act {(number, number), (number, number), (number, number)...nullable}

### 受信

{id, board, res}が返ってくる。board は 5 x 3 の配列であり、None, "O_A", "S_A", "R_A", "U_A", "O_D", "S_D", "R_D", "U_D", "B_N"のうちいずれかが入っている。
A が入っている方が手前側のコマとする。
