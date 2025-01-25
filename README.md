# monapa

`monapa` is a lightweight library of monadic parser combinators for Rust, heavily inspired by [Megaparsec](https://hackage.haskell.org/package/megaparsec).

do記法を搭載したパーサコンビネータライブラリ。


## Examples

`examples`ディレクトリ以下にはサンプルプログラムが用意されています。

```bash
cargo run --example cfg1
```

などとして実行できます。


## Limitations and Known Issues

- 先読みができない（すぐに対応予定）
- packrat parsingには未対応
- `pdo!` マクロ内で補完が効かない
