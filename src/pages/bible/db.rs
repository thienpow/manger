use rexie::*;
use serde::Serialize;
use wasm_bindgen::JsValue;
use super::store::{VerseItem, BibleBookItem};

#[derive(Debug, Serialize)]
struct BookRequest {
    pub book_id: u32,
    pub book_name: String,
    pub chapters: u32,
}

#[derive(Debug, Serialize)]
struct VerseRequest<'a> {
    book_id: u32,
    chapter: u32,
    verse: u32,
    text: &'a str
}

pub async fn _close_and_delete_db(rexie: Rexie) {
    rexie.close();
    assert!(Rexie::delete("manger").await.is_ok());
}

pub async fn build_database() -> Result<Rexie> {
    // Create a new database
    let rexie = Rexie::builder("manger")
        // Set the version of the database to 1.0
        .version(1)
        // Add an object store named `bibleBooks`
        .add_object_store(
            ObjectStore::new(format!("bibleBooks-en/kjv").as_str())
                // Set the key path to `id`
                .key_path("id")
                // Enable auto increment
                .auto_increment(true)
        )
        .add_object_store(
            ObjectStore::new(format!("bibleVerses-en/kjv").as_str())
                // Set the key path to `id`
                .key_path("id")
                // Enable auto increment
                .auto_increment(true)
        )
        .add_object_store(
            ObjectStore::new(format!("bibleBooks-zh/cuv_t").as_str())
                // Set the key path to `id`
                .key_path("id")
                // Enable auto increment
                .auto_increment(true)
        )
        .add_object_store(
            ObjectStore::new(format!("bibleVerses-zh/cuv_t").as_str())
                // Set the key path to `id`
                .key_path("id")
                // Enable auto increment
                .auto_increment(true)
        )
        .add_object_store(
            ObjectStore::new(format!("bibleBooks-zh/cnv_t").as_str())
                // Set the key path to `id`
                .key_path("id")
                // Enable auto increment
                .auto_increment(true)
        )
        .add_object_store(
            ObjectStore::new(format!("bibleVerses-zh/cnv_t").as_str())
                // Set the key path to `id`
                .key_path("id")
                // Enable auto increment
                .auto_increment(true)
        )
        // Build the database
        .build()
        .await?;
 
     Ok(rexie)
}

pub async fn check_books_downloaded(rexie: &Rexie, bible: String) -> Result<bool> {
    let store_name = format!("bibleBooks-{}", bible);
    let transaction = rexie.transaction(&[store_name.clone().as_str()], TransactionMode::ReadOnly);
    assert!(transaction.is_ok());
    let transaction = transaction.unwrap();

    let bible_books = transaction.store(store_name.clone().as_str());
    assert!(bible_books.is_ok());
    let bible_books = bible_books.unwrap();

    let id: u32 = 66;
    let book_js = bible_books.get(&id.into()).await?;
    if book_js.is_undefined() {
        Ok(false)
    } else {
        Ok(true)
    }

}

pub async fn get_books(rexie: &Rexie, bible: String) -> Result<Vec<BibleBookItem>> {
    let store_name = format!("bibleBooks-{}", bible);
    let transaction = rexie.transaction(&[store_name.clone().as_str()], TransactionMode::ReadOnly);
    assert!(transaction.is_ok());
    let transaction = transaction.unwrap();

    let bible_books = transaction.store(store_name.clone().as_str());
    assert!(bible_books.is_ok());
    let bible_books = bible_books.unwrap();

    let books_js: Vec<JsValue> = bible_books
        .get_all(None, None, None, None)
        .await?
        .into_iter()
        .map(|pair| pair.1)
        .collect();

    let books: Vec<BibleBookItem> = books_js
        .into_iter()
        .map(|book| {
            let v = serde_wasm_bindgen::from_value(book).unwrap();
            v
        })
        .collect();

    Ok(books)
}

pub async fn append_all_books(rexie: &Rexie, bible: String, books: Vec<BibleBookItem>) -> Result<u32> {
    // Create a new read-write transaction
    let store_name = format!("bibleBooks-{}", bible);
    let transaction = rexie.transaction(&[store_name.clone().as_str()], TransactionMode::ReadWrite)?;

    // Get the `bibleBooks` store
    let bible_books = transaction.store(store_name.clone().as_str())?;

    let mut i: u32 = 0;
    for book in &books {
        i = i  + 1;
        // Create an verse
        
        /*
        let book = BookRequest { 
            book_id: book.book_id, 
            book_name: book.book_name.clone(), 
            chapters: book.chapters
        };
         */

        // Convert it to `JsValue`
        let book_js = serde_wasm_bindgen::to_value(&book).unwrap();

        //console::log_1(&"here trying to add".into());
        //console::log_1(&verses_js);
        // Add the verse to the store
        bible_books.add(&book_js, None).await?;
        
    };
    
    // Waits for the transaction to complete
    transaction.done().await?;

    Ok(num_traits::cast(i).unwrap())
}


pub async fn check_verses_downloaded(rexie: &Rexie, bible: String) -> Result<bool> {
    let store_name = format!("bibleVerses-{}", bible);
    let transaction = rexie.transaction(&[store_name.clone().as_str()], TransactionMode::ReadOnly);
    assert!(transaction.is_ok());
    let transaction = transaction.unwrap();

    let bible_verses = transaction.store(store_name.clone().as_str());
    assert!(bible_verses.is_ok());
    let bible_verses = bible_verses.unwrap();

    let id: u32 = 30000;
    let verse_js = bible_verses.get(&id.into()).await?;
    if verse_js.is_undefined() {
        Ok(false)
    } else {
        Ok(true)
    }

}


pub async fn get_full_verses(rexie: &Rexie, bible: String) -> Result<Vec<VerseItem>> {
    let store_name = format!("bibleVerses-{}", bible);
    let transaction = rexie.transaction(&[store_name.clone().as_str()], TransactionMode::ReadOnly);
    assert!(transaction.is_ok());
    let transaction = transaction.unwrap();

    let bible_verses = transaction.store(store_name.clone().as_str());
    assert!(bible_verses.is_ok());
    let bible_verses = bible_verses.unwrap();

    let verses_js: Vec<JsValue> = bible_verses
        .get_all(None, None, None, None)
        .await?
        .into_iter()
        .map(|pair| pair.1)
        .collect();

    let verses: Vec<VerseItem> = verses_js
        .into_iter()
        .map(|verse| {
            let v = serde_wasm_bindgen::from_value(verse).unwrap();
            v
        })
        .collect();

    Ok(verses)
}

pub async fn append_all_verses(rexie: &Rexie, bible: String, verses: Vec<VerseItem>) -> Result<u32> {
    // Create a new read-write transaction
    let store_name = format!("bibleVerses-{}", bible);
    let transaction = rexie.transaction(&[store_name.clone().as_str()], TransactionMode::ReadWrite)?;

    // Get the `bible-verses` store
    let bible_verses = transaction.store(store_name.clone().as_str())?;

    let mut i: u32 = 0;
    for verse in &verses {
        i = i  + 1;
        // Create an verse
        
        /*
        let verse = VerseRequest { 
            book_id: verse.book_id, 
            chapter: verse.chapter, 
            verse: verse.verse, 
            text: &verse.text
        };
         */

        // Convert it to `JsValue`
        let verse_js = serde_wasm_bindgen::to_value(&verse).unwrap();

        // Add the verse to the store
        bible_verses.add(&verse_js, None).await?;
        
    };
    
    // Waits for the transaction to complete
    transaction.done().await?;

    Ok(num_traits::cast(i).unwrap())
}