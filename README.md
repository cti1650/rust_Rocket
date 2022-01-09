# rust_rocket_test

## Rocket

### 初期設定

```
curl https://sh.rustup.rs -sSf | /bin/bash -s -- -y --default-toolchain nightly
```

### サーバー起動

```
cargo run
```

## サンプル

<a title="Gitpod" href="https://gitpod.io/#https://github.com/cti1650/rust_rocket_test" rel="nofollow noreferrer noopener" target="_blank" class="btn btn-primary">Gitpodでサンプルを実行</a>

### 本番リンク

[https://rust-rocket-test-cti-tl.herokuapp.com/](https://rust-rocket-test-cti-tl.herokuapp.com/)


```
#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

/// GETがきたときに"Hello, world!"というレスポンスを返す
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])  // ここにルーティングをセットする
        .launch();
}
```

## heroku

### デプロイ時につまづいたこと

- ローカルでは問題なく起動できるのに heroku ではうまく動作しなかった

#### RustConfigを忘れるとデプロイに失敗する

RustConfig

```
VERSION=nightly
```

#### デプロイが成功してもProcfileへのapp nameの指定が誤っていると正常に動作しない

Cargo.tomlで指定したnameをProcfileに記入が必要

Procfile
```text
web: ROCKET_PORT=$PORT ./target/release/{{app name}}
```


## 参考にしたサイト

- [Rust RocketでのWebAPIサーバーの書き方を解説してみる - Qiita](https://qiita.com/yukinarit/items/c5128e67d168b4f39983)

- [Rust Rocket,Dieselを使用した簡単なGET,POST](https://zenn.dev/sgtkuc1118/articles/4571d7960dfbfc)

- [RustのWebアプリ(Rocket)をHerokuにデプロイする - Qiita](https://qiita.com/ovrmrw/items/b316f01f4a936fec9b85)

- [【Heroku】Herokuで環境変数を設定する方法 - Qiita](https://qiita.com/mzmz__02/items/64db94b8fc67ee0a9068)
