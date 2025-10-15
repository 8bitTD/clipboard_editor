## 概要
このリポジトリは、Rustで作った **クリップボードに保存されている画像を編集するツール** のリポジトリになります。

クリックボードに保存された画像に以下の処理が出来ます
* 画像を見ながら解像度の縮小具合を変更出来ます
* 画像で映ってはいけないところに黒板を配置して見えなく出来ます

## ビルド
[Rust](https://rust-lang.org/ja/)をインストールして、
```
git clone https://github.com/8bitTD/clipboard_editor
cd clipboard_editor
cargo run --release
```

## 使い方
クリップボードに画像をコピーした状態でclipboard_editor.exeを実行すると画像が表示されます。
![aaa](https://github.com/user-attachments/assets/e3e2671f-6451-4296-bf91-2175f75c2538)

### 画像を縮小する
マウスホイールを上方向に回転させると画像が縮小します。
### 黒い板を配置する
マウスの左ボタンをドラッグ＆ドロップすると黒い板が配置されます。
諸事情で画像の見せたくない箇所に配置してください。
### Undo
キーボードの ctrl + z
### Redo
キーボードの ctrl + y
### クリップボードに上書きする
キーボードの Enter
### ツールを終了する
キーボードの Esc

## 動作確認
Windows 11
