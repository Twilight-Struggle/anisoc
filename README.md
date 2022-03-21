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

## 実行方法(ランダムAI)

ビルドは次のコマンドで実行可能

```sh
$ docker build --tag anisoc --file Dockerfile .
```

起動は以下の通り。

```sh
$ docker run　--name anisoc_ins -p 8000:8000 anisoc
```

## core - ai インターフェース

backend と ai 両者に対して対等なインターフェースを持つ必要がある。

# python深層学習との結合
## ai - python インターフェース

python の AI が受け取る盤面や合法手はすべて自分は手前側のプレイヤーであるという前提が必要(手前側から 0 オリジンの盤面)。
python 側は np.empty((9, 5, 3), dtype=np.float32)という numpy 配列を受け取って手を出力する。
更に合法手一覧のリストが引数として必要である。合法手は以下のように int に変換して渡す。

合法手は from = 0 ～ 15(0 ～ 14 は盤面、15 はパス), to = 0 ～ 15(同様), kick_to = 0 ～ 18(16 は自分ゴール、18 は相手ゴール)としたとき、
from x (16 x 18) + to x 18 + kick_to がその手を int で表している。
渡す合法手一覧のリストは例えば[12, 536, 1642]などという風になる。

python 側の返り値は int 化した合法手とする。

## core

## docker-composeを利用してネットワーク
同一のdocker-composeの同services内にdockerインスタンスの設定を書けば自動でネットワークが構成される。
これを利用してgRPC用のネットワークも構成した。
なおコメントアウトされているttyは、trueにするとdocker-composeで起動した後、`docker-compose exec サービス名 bash`でシェルに入れるようにコンテナを永続化させておくためのものである。

linksでanisocがanipyに依存することを示し、portsで50051ポートを開放することでgRPCでの通信が可能になる(このとき指定するホスト側の対応するポートは何でもいい)。

# GCPでの実行
## AIコンテナの自宅サーバでの起動
AI用gRPCサーバコンテナは今まで通りports設定をして起動するだけだが、GCPからグローバルIPを使用してgRPCクライアントが自宅PCへアクセスできるようにポート開放を行う必要がある。
なお自宅にグローバルIPが割り当てられていない形式の場合、GCPのVPCの間にVPNを確立して…といった作業が必要な模様。

GCP側のgRPCクライアントには`自宅のグローバルIP + ポート番号`を指定することで自宅PCのコンテナ内のAI用gRPCサーバにアクセスすることができる。
経路は「GCP`?.?.?.?:5555`」→「自宅ルータ`?.?.?.?:5555`」→「ポートフォワーディング」→「`192.168.?.?:4444`」→
「Windowsファイヤーウォール」→「`192.168.?.?:4444`」→「docker runtimes」→「`0.0.0.0:50051`」→「gRPCサーバ」。

## グローバルIP
以下のサイトで調べられる。
https://www.cman.jp/network/support/go_access.cgi

### ポート開放
まずルータのポートマッピングあるいはポートフォワーディング設定で、受け付けたいポート番号とそのポートにアクセスが来た場合の接続先(この場合自宅PC)のローカルIPを設定する。
これにより`グローバルIP:受け付けたいポート番号`に来たアクセスが`自宅PCローカルIP:受け付けたいポート番号`に転送されるようになる。

次に以下の通りWindowsファイヤーウォールで許可設定を行う。
https://support.borndigital.co.jp/hc/ja/articles/360002711593-Windows10%E3%81%A7%E7%89%B9%E5%AE%9A%E3%81%AE%E3%83%9D%E3%83%BC%E3%83%88%E3%82%92%E9%96%8B%E6%94%BE%E3%81%99%E3%82%8B

ポート開放ができているかの確認は以下のサイトで実施できる。
https://www.akakagemaru.info/port/wrx.php?port=8080

## GCPへのデプロイ
ローカルのdockerからArtifact Registryへpushし、そこからCloud Runにデプロイする。

- PROJECT_ID: anisoc???
- REGION: asia-northeast1

### ソースコードの変更
anicore/src/aigrpc.rsのクライアント部のアドレスとポートを変更する必要がある。アドレス部は上述の`http://グローバルIP`に、ポートを適当なものに変更する。
またaniweb/configuration/base.ymlのポートを8080に変更。

### GCPの初期設定
https://cloud.google.com/run/docs/tutorials/system-packages?hl=ja#before-you-begin

1. アカウント作成
1. gcloundインストール(https://laboradian.com/install-google-cloud-sdk-on-wsl-ubuntu/)
1. `$ gcloud components update`


### GCPプロジェクトの初期設定
https://cloud.google.com/run/docs/tutorials/system-packages?hl=ja#before-you-begin
https://cloud.google.com/run/docs/tutorials/system-packages?hl=ja#setting-up-gcloud
https://cloud.google.com/artifact-registry/docs/docker/store-docker-container-images?hl=ja

1. Google Cloud Console の [プロジェクト セレクタ] ページで、Google Cloud プロジェクトを選択または作成します。
1. Cloud プロジェクトに対して課金が有効になっていることを確認します。詳しくは、プロジェクトで課金が有効になっているかどうかを確認する方法をご覧ください。
1. Cloud Run Admin API を有効にします。
1. Artifact Registry API を有効にします。
1. デフォルト プロジェクトを設定します。`$ gcloud config set project PROJECT_ID`
1. 選択したリージョン向けに gcloud を構成します。`$ gcloud config set run/region REGION`

### Dockerリポジトリ作成
1. https://cloud.google.com/artifact-registry/docs/docker/store-docker-container-images?hl=ja#create
    1. `gcloud artifacts repositories create anisoc-docker-repo --repository-format=docker --location=asia-northeast1 --description="Docker repository"`
    1. `gcloud artifacts repositories list`
1. https://cloud.google.com/artifact-registry/docs/docker/store-docker-container-images?hl=ja#auth
    1. `gcloud auth configure-docker asia-northeast1-docker.pkg.dev`

### Dockerイメージビルド
https://cloud.google.com/run/docs/building/containers?hl=ja#docker
1. `docker build --tag anisoc_push --file Dockerfile .`
1. `docker tag anisoc_push asia-northeast1-docker.pkg.dev/$PROJECT/anisoc-docker-repo/anisoc`

### Artifact Registryにpush
https://cloud.google.com/artifact-registry/docs/docker/store-docker-container-images?hl=ja#tag
1. `docker push asia-northeast1-docker.pkg.dev/$PROJECT/anisoc-docker-repo/anisoc`

### Cloud Runへのデプロイ
https://cloud.google.com/artifact-registry/docs/integrate-cloud-run?hl=ja#console

### Dockerリポジトリクリーンアップ
https://cloud.google.com/artifact-registry/docs/docker/store-docker-container-images?hl=ja#clean-up

### GCPプロジェクトの削除
https://cloud.google.com/run/docs/tutorials/system-packages?hl=ja#delete-project
上記サイトの説明の通り。