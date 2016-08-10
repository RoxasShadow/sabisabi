錆び錆び
=======
[![](https://meritbadge.herokuapp.com/sabisabi)](https://crates.io/crates/sabisabi)

錆び錆び is a memorization software that uses Anki collections to allow players to
guess what's on the other side of a card by choosing the right one out of the proposed answers.

Usage
-----
First of all, export your anki deck to a plain text file through the official client.

Compile 錆び錆び by running `$ cargo build --release` and execute it with a command
like `$ ./sabisabi --path ../kanji.txt -side back --format notes --tag 日記 --strip-parents`.

You can get help by running `$ ./sabisabi --help`.

During its execution, press `Ctrl` + `C` to quit.

Example
-------
```
┌[giovanni@lifestream-3] [/dev/ttys001] [master]
└[~]> sabisabi -s front -p ~/Desktop/kanji.txt
da quando?
  1) 目
  2) 行く
  3) 曜日
  4) いつから
4
Your answer is correct!

per cui
  1) 作る
  2) だから
  3) と言う
  4) 土曜日
4
Your answer is wrong.

おおきい (grande)
  1) 磨く
  2) 大きい
  3) ２時間後
  4) 29日
大きい
Your answer is correct!

おとす (far cadere, tra.)
  1) 変わる
  2) 火曜日
  3) 土曜日
  4) 落とす
落
Your answer is invalid. Please retry.

きゅう９
  1) ボールペン
  2) 言う
  3) 十
  4) 九
```
