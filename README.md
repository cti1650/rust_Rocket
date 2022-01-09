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

## 参考にしたサイト

- [Rust RocketでのWebAPIサーバーの書き方を解説してみる - Qiita](https://qiita.com/yukinarit/items/c5128e67d168b4f39983)
