# anisoc_web

このアプリケーション起動時に anisoc_core やフロントエンドのセットアップを行う。
ゲームインスタンスもこの中に保持。

## core との接続

今回はプラン 2 を採用した。

### プラン 1

Agent トレイトを実装する WebPlayer にアクション登録用のメソッドと action が呼び出されたとき用のメソッドを実装する。
→GUI 同士(async 同士)で対戦するなら必須だが過剰な気がする。

### プラン 2

AI をコアループ内に組み込んでしまい、web 側からは同期関数に見えるようにする。
