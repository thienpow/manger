use serde::{Deserialize, Serialize};
use reqwasm::http::Request;

use crate::context::BibleBookItem;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct TableOfContents {
    pub books: Vec<BibleBookItem>
}

//const API_BASE_URL: &str = "https://manger.aadi.my";

pub async fn get_toc() -> Result<TableOfContents, reqwasm::Error> {

    let url = format!("/bible/{}", "toc.json");
    let resp = Request::get(&url).send().await?;

    let result = resp.json::<TableOfContents>().await?;
    Ok(result)
}