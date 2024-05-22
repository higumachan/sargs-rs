# sargs

` sargs`
は、xargsの代替として設計されたコマンドラインツールです。xargsと異なり、入力側のプログラムが終了する前に、ストリーミングで後続のコマンドを実行することができます。これにより、リアルタイムでのデータ処理が可能になり、効率的なワークフローを構築できます。

![Crates.io Version](https://img.shields.io/crates/v/sargs-cmd)

# 言語

- [English](README.md)
- [日本語](README.ja.md)

## 特徴

- **非同期実行:** 入力側のプログラムが完全に終了するのを待たずに、データが利用可能になり次第、後続のコマンドを実行します。
- **高効率:** ストリーミング処理により、大量のデータでもメモリ消費を抑えながら迅速に処理できます。
- **柔軟性:** 様々なシェル環境やコマンドラインツールと組み合わせて使用でき、多様な用途に対応します。

## インストール方法

現在、` sargs` はRustのパッケージマネージャであるCargoを使用してインストールできます。

```sh
cargo install sargs-cmd
```

## 使用方法

基本的な使用方法はxargsと似ていますが、` sargs` はデータがストリームとして後続のコマンドに渡される点が異なります。以下は、標準入力からデータを受け取り、各行をechoコマンドに渡す例です。

```sh
cat example.txt | sargs echo
```

上記のコマンドは `example.txt` の各行を読み込み、読み込まれ次第に `echo` コマンドに渡します。これにより、ファイルの内容がリアルタイムで処理されます。
この例の場合は、xargsとの挙動の差はわかりにくいです。

例えば以下のように、catの途中に1行を読み込んで1秒待つようなコマンドが挟まると、xargsでは全ての行を読み込んでからすべての行に対して実行しますが、sargsでは1行読み込むたびに即座に実行します。

```sh
cat example.txt | slow_pass_command | sargs echo
```

## 設定とオプション

` sargs` はいくつかのカスタマイズ可能なオプションを提供しています。コマンドラインオプションを通じて挙動を調整できます。全てのオプションとその説明については、以下のコマンドを使用して確認できます。

```sh
sargs --help

Usage: sargs [OPTIONS] [ARGS]...

Arguments:
  [ARGS]...  

Options:
  -I <INPUT_PLACEHOLDER>           
      --buffer-size <BUFFER_SIZE>  
  -h, --help                       Print help
  -V, --version                    Print version
```

### INPUT_PLACEHOLDER

`-I` または `--input-placeholder` オプションを使用すると、後続のコマンドに渡されるデータのプレースホルダを指定できます。

```sh
cat example.txt | sargs -I __INPUT__ echo __INPUT__
```

で、`example.txt` の各行が `echo` コマンドに渡される際に、`__INPUT__` が各行の内容に置き換えられます。

### BUFFER_SIZE

`--buffer-size` オプションを使用すると、出力コマンドのバッファの個数を指定できます。デフォルトは `128` です。
後続のコマンドが遅いくて、`Buffer full`のエラーが発生する場合は、このオプションを使用してバッファの個数を増やすことができます。

## コントリビューション

` sargs` はオープンソースプロジェクトであり、コミュニティの貢献を歓迎しています。バグ報告、機能提案、プルリクエストなどをGitHubリポジトリを通じて行ってください。

## ライセンス

` sargs` は [MITライセンス](https://opensource.org/licenses/MIT) の下で公開されています。