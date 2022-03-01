use serde::{Deserialize, Serialize};
use reqwasm::http::Request;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct TableOfContents {
    pub books: Vec<Book>
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct Book {
    pub book_id: i32,
    pub book_name: String,
    pub chapters: i32,
}

//const API_BASE_URL: &str = "https://manger.aadi.my";

pub async fn get_toc() -> Result<TableOfContents, reqwasm::Error> {

    let url = format!("/bible/{}", "toc.json");
    let resp = Request::get(&url).send().await?;

    let result = resp.json::<TableOfContents>().await?;
    Ok(result)
}