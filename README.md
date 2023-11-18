# rust-cp
Rustによるcpコマンド。標準入力を標準出力にコピーします。

# details
Rustによる習作。『ソフトウェア作法』に倣って最もシンプルなプログラムを作りました。

最初にBufReader型の変数readerを作ります。この変数は標準入力に結び付けられています。

```rust
let reader = BufReader::new(stdin());
```

次にreaderから各行に対応するイテレーターを取り出して、for文で回します。lineがイテレーターで、入力のストリームの先頭から順にイテレータが生成されます。

```rust
for line in reader.lines() {
```

このイテレータを使って、for文の中で各行を印字します。イテレータはResult型を返すので、Unwrap()メソッドを使って成功時の値（String型）を取り出しています。

```rust
println!("{}", line.unwrap());
```

ところで、for文の引数である reader.lines() ですが、これはstd::io::BufReader型のmethodではありません。std::io::BufRead型のメソッドです。

したがって、プログラムの冒頭の宣言は次のようになります。
```rust
use std::io::{stdin, BufRead, BufReader};
```

# ライセンス
[MIT License](LICENSE)
