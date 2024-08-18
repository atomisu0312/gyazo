# ビルドステージ
FROM rust:1.80.1-slim-bullseye AS builder

# 必要なライブラリをインストール
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# 作業ディレクトリを設定
WORKDIR /usr/src/myapp

# Cargo.tomlとCargo.lockをコピー
COPY Cargo.toml Cargo.lock ./

# ソースコードをコピー
COPY src ./src

# 依存関係をダウンロード
RUN cargo build --release

# プロジェクトをビルド
RUN cargo install --path .

# 実行ステージ
FROM debian:bullseye-slim

# 必要なライブラリをインストール
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# ビルドステージから実行可能ファイルをコピー
COPY --from=builder /usr/local/cargo/bin/stock-cli /usr/local/bin/stock-cli

# 実行可能ファイルを実行
CMD ["sh", "-c", "stock-cli ${SYMBOL}"]