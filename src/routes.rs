// JSONを返すのに必要
use rocket_contrib::json::Json;

use crate::models::SampleJson;

#[get("/")]
pub fn index() -> Json<SampleJson> {
    Json(SampleJson {
        text: "Hello World!!".into(),
    })
}

/// GETがきたときに"Hello, world!"というレスポンスを返す
#[get("/sampleword")]
pub fn sampleword() -> &'static str {
    "Hello, world!"
}