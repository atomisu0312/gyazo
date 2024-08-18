# 実行方法
## Dockerイメージをビルド
docker build -t hello_world .

## 環境変数を設定せずにコンテナを実行
docker run --rm hello_world

## 環境変数を設定してコンテナを実行
docker run --rm -e TEST_ECHO="This is a test" hello_world