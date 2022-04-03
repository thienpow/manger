
use serde::{Deserialize, Serialize};
use reqwasm::http::Request;

use crate::pages::bible::store::{BibleBookItem, MetadataItem, VerseItem};

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct Books {
    pub books: Vec<BibleBookItem>
}
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct BookData {
    pub metadata: MetadataItem,
    pub verses: Vec<VerseItem>
}

//const API_BASE_URL: &str = "https://manger.aadi.my";

pub async fn download_books(bible: String) -> Result<Books, reqwasm::Error> {

    let url = format!("/bible/{}/toc.json", bible);
    let resp = Request::get(&url).send().await?;

    let result = resp.json::<Books>().await?;
    Ok(result)
}

pub async fn download_full_verses(bible: String) -> Result<BookData, reqwasm::Error> {

    let url = format!("/bible/{}/full.json", bible);
    let resp = Request::get(&url).send().await?;

    let result = resp.json::<BookData>().await?;

    Ok(result)
}

pub async fn _get_book_data(bible: String, book: u32) -> Result<BookData, reqwasm::Error> {

    //console::log_1(&"get_book_data".into());
    let book = format!("{}.json", book);

    let url = format!("/bible/{}/{}", bible, book);
    let resp = Request::get(&url).send().await?;

    let result = resp.json::<BookData>().await?;

    //console::log_1(&"done get_book_data".into());
    Ok(result)
}