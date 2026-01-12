# embedded-graphics でピクセルアート風時計を実装する計画

## 概要

現在の eframe/egui ベースの時計アプリを、embedded-graphics
を使ったピクセルアート風の表示に変更する。

## ターゲット環境

- Raspberry Pi OS (Linux)
- RC050S ディスプレイ (800x480 HDMI)

## アプローチ: minifb + embedded-graphics
理由
- minifb: クロスプラットフォームのウィンドウ/フレームバッファライブラリ
  - Windows でも開発・テスト可能
  - Raspberry Pi OS でもそのまま動作
  - 軽量で依存関係が少ない
- embedded-graphics: 2D描画プリミティブを提供
  - ピクセルアート風のフォント（例: profont, u8g2-fonts）が利用可能
  - シンプルな描画API

## 実装計画

1. 依存関係の変更 (Cargo.toml)

```
# 削除
eframe = "0.33.3"

# 追加
minifb = "0.29"
embedded-graphics = "0.8"
embedded-graphics-core = "0.4"
u8g2-fonts = "0.5"  # ピクセルフォント
```

2. 描画ターゲットの実装

minifb の Window を embedded-graphics の DrawTarget として使えるアダプタを作成。

3. ピクセルアート風UIの実装

- 時刻: 大きなピクセルフォントで表示
- 日付: 小さめのピクセルフォントで表示
- 気温: アイコン + 数値

4. メインループの変更

- minifb のウィンドウを作成 (800x480)
- フレームバッファに embedded-graphics で描画
- 定期的に更新

## 変更するファイル

- Cargo.toml - 依存関係の変更
- src/main.rs - 全面的に書き換え
- src/weather.rs - 変更なし（そのまま使用）

## 検証方法

1. Windows環境でビルド・実行してウィンドウ表示を確認
2. cargo build --release でビルド
3. 時刻・日付・気温が表示されることを確認
4. 画面が1分ごとに更新されることを確認