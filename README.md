# H.O.S BABEL スクリーンセーバー

Flash ベースの `HOS_BABEL.scr` を Rust で再実装した、Windows 11 対応のスクリーンセーバーです。

## 📥 ダウンロード

### ビルド済みファイル（Windows 用）

Windows ユーザーはビルド済みの `.scr` ファイルを直接ダウンロードできます：

**[HOS_BABEL.scr をダウンロード](./releases/HOS_BABEL.scr)** _(準備中)_

または、[Releases](../../releases) ページから最新版をダウンロードしてください。

### インストール方法（Windows）

1. `HOS_BABEL.scr` をダウンロード
2. ファイルを右クリックして「インストール」を選択
3. または `C:\Windows\System32` にコピー
4. Windows の設定から「スクリーンセーバー」を開き、"H.O.S BABEL Screensaver" を選択

## 概要

画面上にランダムな位置に "BABEL " という赤色のテキストを表示し続けるシンプルなスクリーンセーバーです。元々は Flash アニメーションでしたが、Windows 11 では Flash のサポートが終了したため、Rust で再実装しました。

### 特徴

- ✅ Windows 11 完全対応（Flash 不要）
- ✅ 実際のディスプレイサイズに自動対応
- ✅ ボーダーレス表示
- ✅ 軽量・高速動作
- ✅ クロスプラットフォーム（macOS / Linux でも動作）

## ビルド方法

### macOS / Linux でのビルド

```bash
cargo build --release
```

実行ファイルは `target/release/hos_screensaver` に生成されます。

### Windows でのビルド

Windows 環境では、以下のコマンドでビルドします：

```bash
cargo build --release --target x86_64-pc-windows-msvc
```

実行ファイルは `target/x86_64-pc-windows-msvc/release/hos_screensaver.exe` に生成されます。

### スクリーンセーバーファイル（.scr）の作成

Windows でビルドした後、拡張子を `.scr` に変更します：

```cmd
copy target\x86_64-pc-windows-msvc\release\hos_screensaver.exe HOS_BABEL.scr
```

## コマンドライン引数

Windows スクリーンセーバーの標準的な引数に対応しています：

| 引数 | 動作 |
|------|------|
| `/s` または引数なし | スクリーンセーバーモード（フルスクリーン） |
| `/p <HWND>` | プレビューモード（設定画面の小窓で実行） |
| `/c` | 設定ダイアログ（このスクリーンセーバーには設定項目なし） |

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

### 基本情報

- **言語**: Rust (Edition 2021)
- **GUI ライブラリ**: minifb 0.27 (軽量フレームバッファライブラリ)
- **描画**: カスタム 8x16 ビットマップフォント

### パフォーマンス

- **フレームレート**: 60 FPS
- **テキスト追加間隔**: 10ms ごと
- **テキスト表示時間**: 100 秒間
- **色**: 赤色 (0xFF0000)

### ウィンドウ設定

- **ボーダーレス**: 常に有効（枠なし表示）
- **タイトルバー**: 非表示
- **最前面表示**: 有効
- **リサイズ**: 不可

### 画面サイズ対応

- **Windows**: `GetSystemMetrics` API で実際のディスプレイサイズを自動取得
- **macOS / Linux**: デフォルトで 1920x1080
- **プレビューモード**: 固定サイズ 800x600

## ディレクトリ構成

```
H.O.S_screensaver/
├── Cargo.toml          # プロジェクト設定
├── Cargo.lock          # 依存関係ロック
├── README.md           # このファイル
├── src/
│   ├── main.rs         # メインプログラム
│   ├── HOS_BABEL.scr   # 元の Flash ベーススクリーンセーバー（参考用）
│   └── HOS.lzh         # 元の HOS プログラムのアーカイブ
└── target/             # ビルド成果物（自動生成）
```

## クロスコンパイル（macOS から Windows 向けにビルド）

macOS から Windows 用の実行ファイルをビルドする場合：

### 1. Windows ターゲットを追加

```bash
rustup target add x86_64-pc-windows-gnu
```

### 2. mingw-w64 をインストール

```bash
brew install mingw-w64
```

### 3. ビルド

```bash
cargo build --release --target x86_64-pc-windows-gnu
```

生成されたファイル: `target/x86_64-pc-windows-gnu/release/hos_screensaver.exe`

## トラブルシューティング

### ボーダーレス表示にならない

- minifb のバージョンや OS によっては、完全なボーダーレス表示が難しい場合があります
- 現在、以下の設定を適用しています：
  - `borderless = true`
  - `title = false`
  - `topmost = true`

### Windows でビルドエラーが出る

- `GetSystemMetrics` のインポートエラーが出る場合：
  - `Cargo.toml` に `Win32_UI_WindowsAndMessaging` 機能が含まれているか確認してください
  - 既に修正済みですが、古いバージョンからアップデートする場合は `cargo clean` を実行してください

## 依存関係

- `minifb = "0.27"` - ウィンドウ管理とフレームバッファ描画
- `rand = "0.8"` - ランダム位置生成
- `windows = "0.58"` - Windows API（Windows のみ）

## ライセンス

このプロジェクトは元の HOS_BABEL.scr を再実装したものです。

## 謝辞

元の プログラムおよびHOS_BABEL スクリーンセーバーの作者お二人に感謝します。