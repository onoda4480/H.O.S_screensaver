# H.O.S BABEL スクリーンセーバー

Flash ベースの `HOS_BABEL.scr` を Rust で再実装した、Windows 11 対応のスクリーンセーバーです。

## 概要

画面上にランダムな位置に "BABEL " という赤色のテキストを表示し続けるシンプルなスクリーンセーバーです。元々は Flash アニメーションでしたが、Windows 11 では Flash のサポートが終了したため、Rust で再実装しました。

## ビルド方法

### macOS / Linux でのビルド

```bash
cargo build --release
```

### Windows でのビルド

Windows 環境では、以下のコマンドでビルドします：

```bash
cargo build --release --target x86_64-pc-windows-msvc
```

ビルド後、実行ファイルは `target/release/hos_screensaver` (または Windows の場合は `target/x86_64-pc-windows-msvc/release/hos_screensaver.exe`) に生成されます。

## Windows スクリーンセーバーとしてインストール

1. Windows 環境でビルドした `hos_screensaver.exe` の拡張子を `.scr` に変更します：
   ```cmd
   copy target\x86_64-pc-windows-msvc\release\hos_screensaver.exe HOS_BABEL.scr
   ```

2. `.scr` ファイルを `C:\Windows\System32` にコピーするか、右クリックして「インストール」を選択します。

3. Windows の設定から「スクリーンセーバー」を開き、"H.O.S BABEL Screensaver" を選択します。

## コマンドライン引数

Windows スクリーンセーバーの標準的な引数に対応しています：

- `/s` または引数なし: スクリーンセーバーモード（フルスクリーン）
- `/p <HWND>`: プレビューモード（設定画面の小窓で実行）
- `/c`: 設定ダイアログ（このスクリーンセーバーには設定項目なし）

## テスト実行

### プレビューモード（ウィンドウ表示）

```bash
cargo run --release -- /p
```

### フルスクリーンモード

```bash
cargo run --release
```

または

```bash
cargo run --release -- /s
```

**終了方法**: ESC キーまたは任意のキーを押すと終了します。

## 技術仕様

- **言語**: Rust
- **GUI ライブラリ**: minifb (軽量フレームバッファライブラリ)
- **描画**: カスタム 8x16 ビットマップフォント
- **フレームレート**: 60 FPS
- **テキスト表示間隔**: 100ms ごと
- **テキスト表示時間**: 5 秒間

## ディレクトリ構成

```
H.O.S_screensaver/
├── Cargo.toml          # プロジェクト設定
├── src/
│   └── main.rs         # メインプログラム
├── src/
│   ├── HOS_BABEL.scr   # 元の Flash ベーススクリーンセーバー（参考用）
│   └── HOS.lzh         # 元の HOS プログラムのアーカイブ
└── README.md           # このファイル
```

## クロスコンパイル（macOS から Windows 向けにビルド）

macOS から Windows 用の実行ファイルをビルドする場合：

1. Windows ターゲットを追加：
   ```bash
   rustup target add x86_64-pc-windows-gnu
   ```

2. mingw-w64 をインストール：
   ```bash
   brew install mingw-w64
   ```

3. ビルド：
   ```bash
   cargo build --release --target x86_64-pc-windows-gnu
   ```

## ライセンス

このプロジェクトは元の HOS_BABEL.scr を再実装したものです。