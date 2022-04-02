use rexie::*;
use serde::Serialize;
use wasm_bindgen::JsValue;
use web_sys::console;
use super::store::{VerseItem};

#[derive(Debug, Serialize)]
struct VerseRequest<'a> {
    book_id: u32,
    chapter: u32,
    verse: u32,
    text: &'a str
}

pub async fn close_and_delete_db(rexie: Rexie) {
    rexie.close();
    assert!(Rexie::delete("manger").await.is_ok());
}

pub async fn build_database(bible: String) -> Result<Rexie> {
    // Create a new database
    let bible_str = format!("bible-{}", bible);
    let rexie = Rexie::builder("manger")
        // Set the version of the database to 1.0
        .version(1)
        // Add an object store named `employees`
        .add_object_store(
            ObjectStore::new(bible_str.clone().as_str())
                // Set the key path to `id`
                .key_path("id")
                // Enable auto increment
                .auto_increment(true)
                // Add an index named `email` with the key path `email` with unique enabled
                //.add_index(Index::new("email", "email").unique(true)),
        )
        // Build the database
        .build()
        .await?;
 
     // Check basic details of the database
     //assert_eq!(rexie.name(), "manger");
     //assert_eq!(rexie.version(), 1.0);
     //assert_eq!(rexie.store_names(), vec!["bible-en/kjv"]);
 
     Ok(rexie)
}

pub async fn check_bible_downloaded(rexie: &Rexie, bible: String) -> Result<bool> {
    let bible_str = format!("bible-{}", bible);
    let transaction = rexie.transaction(&[bible_str.clone().as_str()], TransactionMode::ReadOnly);
    assert!(transaction.is_ok());
    let transaction = transaction.unwrap();

    let verses = transaction.store(bible_str.clone().as_str());
    assert!(verses.is_ok());
    let verses = verses.unwrap();

    let id: u32 = 31102;
    let verse = verses.get(&id.into()).await?;
    //console::log_1(&"check_bible_downloaded".into());
    if verse.is_undefined() {
        //console::log_1(&"verse is_undefined".into());
        Ok(false)
    } else {
        //console::log_1(&"verse got something".into());
        //console::log_1(&verse);
        Ok(true)
    }

}


pub async fn get_full_verses(rexie: &Rexie, bible: String) -> Result<Vec<VerseItem>> {
    let bible_str = format!("bible-{}", bible);
    let transaction = rexie.transaction(&[bible_str.clone().as_str()], TransactionMode::ReadOnly);
    assert!(transaction.is_ok());
    let transaction = transaction.unwrap();

    let verses = transaction.store(bible_str.clone().as_str());
    assert!(verses.is_ok());
    let verses = verses.unwrap();

    let verses: Vec<JsValue> = verses
        .get_all(None, None, None, None)
        .await?
        .into_iter()
        .map(|pair| pair.1)
        .collect();

    let verses: Vec<VerseItem> = verses
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
    let bible_str = format!("bible-{}", bible);
    let transaction = rexie.transaction(&[bible_str.clone().as_str()], TransactionMode::ReadWrite)?;

    // Get the `employees` store
    let bible = transaction.store(bible_str.clone().as_str())?;

    let mut i: u32 = 0;
    for verse in &verses {
        i = i  + 1;
        // Create an verse
        
        let verse = VerseRequest { 
            book_id: verse.book_id, 
            chapter: verse.chapter, 
            verse: verse.verse, 
            text: &verse.text
        };

        // Convert it to `JsValue`
        let verses_js = serde_wasm_bindgen::to_value(&verse).unwrap();

        //console::log_1(&"here trying to add".into());
        //console::log_1(&verses_js);
        // Add the verse to the store
        bible.add(&verses_js, None).await?;
        
    };
    
    // Waits for the transaction to complete
    transaction.done().await?;

    Ok(num_traits::cast(i).unwrap())
}