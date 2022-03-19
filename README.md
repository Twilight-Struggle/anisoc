# anisoc

どうぶつサッカーのゲーム機能を実装した anisoc_core、
人間と対戦する用の anisoc_back と anisoc_front、そして対戦用 AI の anisoc_ai(仮)を実装してみる。
anisoc_back - anisoc_core - anisoc_ai の間は共通のインターフェースを持つのが望ましい。
anisoc_ai は python との接続を橋渡しするだけの機能になるのが良いだろう。
またボードゲームのルールは anisoc_core ですべて管理する。

## front

必要なコンポーネントは「ゲーム開始/リセット」、「5 x 3 のボード」、「移動実行」、「移動キャンセル」ボタンと、
「現在の状態」ラベルである。

- 「5 x 3 のボード」... 3 回までクリックできる。
- 「現在の状態」... 現在の移動クリック系列や、ボードエンジンから帰ってきた表示を返す。
- 「移動キャンセル」... 現在の移動クリック系列を削除する。
- 「移動実行」 ... 移動クリック系列をボードエンジンへ送る。2 回クリック(移動だけ)、3 回クリック(移動 + kick)のときだけ押せる。

## front - back インターフェース

react actix-web 間を JSON post でつなぐ。
「ゲーム開始/リセット」、「移動実行」を押すと backend と通信する。

## backend と backend - core インターフェース

backend は zero2production を主に引用して作成する。追加で unit test などは必要かもしれない。

課題となってくるのはインターフェースである。現在は Mutex<HashMap<Uuid, core::Board>>の形式で web アプリ側が保持しているが、これは 1 人専用である。
ai 側との接続も含む必要がある。

## core - ai インターフェース

backend と ai 両者に対して対等なインターフェースを持つ必要がある。

## ai - python インターフェース

python の AI が受け取る盤面や合法手はすべて自分は手前側のプレイヤーであるという前提が必要(手前側から 0 オリジンの盤面)。
python 側は np.empty((9, 5, 3), dtype=np.float32)という numpy 配列を受け取って手を出力する。
更に合法手一覧のリストが引数として必要である。合法手は以下のように int に変換して渡す。

合法手は from = 0 ～ 15(0 ～ 14 は盤面、15 はパス), to = 0 ～ 15(同様), kick_to = 0 ～ 18(16 は自分ゴール、18 は相手ゴール)としたとき、
from x (16 x 18) + to x 18 + kick_to がその手を int で表している。
渡す合法手一覧のリストは例えば[12, 536, 1642]などという風になる。

python 側の返り値は int 化した合法手とする。

## core

## 実行方法

ビルドは次のコマンドで実行可能

```sh
$ docker build --tag anisoc --file Dockerfile .
```

起動は以下の通り。

```sh
$ docker run　--name anisoc_ins -p 8000:8000 anisoc
```
