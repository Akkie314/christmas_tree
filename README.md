# Christmas Tree

!["個々にテキスト"](./resource/thumbnail.gif)
ターミナルでカラフルなクリスマスツリーを表示するRustプログラムです。

## 特徴

- ランダムに配置される色とりどりのLED
- ターミナル幅に自動調整
- LED間の適切な距離を保証（チェビシェフ距離による配置アルゴリズム）

## 必要要件

- Rust 1.75.0以上（edition 2024対応）
- カラー表示対応のターミナル

## インストール

```bash
git clone <repository-url>
cd christmas_tree
cargo build --release
```

## 使い方

### デフォルト設定で実行（高さ12段）

```bash
cargo run
```

または

```bash
./target/release/christmas_tree
```

### カスタム高さで実行

```bash
cargo run -- 20
```

高さの数値を指定することで、ツリーのサイズを変更できます。

## プロジェクト構成

```
src/
├── main.rs           # エントリーポイント
├── config.rs         # ツリーの設定管理
├── led_generator.rs  # LED位置生成ロジック
└── tree_renderer.rs  # ツリー描画ロジック
```

## 技術仕様

### 使用ライブラリ

- `colored` - ターミナルでのカラー出力
- `terminal_size` - ターミナルサイズの取得
- `rand` - LED配置のランダム化

### アルゴリズム

LED配置には以下の制約を満たすアルゴリズムを使用：
- 各LEDは最小チェビシェフ距離2以上離れて配置
- ツリーの形状内にのみ配置
- ランダム性を保ちながら適切な分散を実現

## 開発

### テスト実行

```bash
cargo test
```

### リリースビルド

```bash
cargo build --release
```

## ライセンス

MIT License

## 作者

Name : Aki314
X : [@Aki_31415926](https://x.com/Aki_31415926)
