use rexie::*;
use super::store::VerseItem;

pub async fn build_database() -> Result<Rexie> {
    // Create a new database
    let rexie = Rexie::builder("manger")
        // Set the version of the database to 1.0
        .version(1)
        // Add an object store named `employees`
        .add_object_store(
            ObjectStore::new("bible-en-kjv")
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
     assert_eq!(rexie.name(), "manger");
     assert_eq!(rexie.version(), 1.0);
     assert_eq!(rexie.store_names(), vec!["bible-en-kjv"]);
 
     Ok(rexie)
 }

pub async fn append_all_verses(rexie: &Rexie, bible: String, verses: Vec<VerseItem>) -> Result<u32> {
    // Create a new read-write transaction
    let transaction = rexie.transaction(&["bible-en-kjv"], TransactionMode::ReadWrite)?;

    // Get the `employees` store
    let bible = transaction.store("bible-en-kjv")?;

    let mut i: u32 = 0;
    for verse in &verses {

        // Create an verse
        /*
        let verse = serde_json::json!({
            "book_id": book_id,
            "chapter": chapter,
            "verse": verse,
            "text": text
        });
        */
        let serialized_verse: String = serde_json::to_string(&verse).unwrap();

        // Convert it to `JsValue`
        let verses_js = serde_wasm_bindgen::to_value(&serialized_verse).unwrap();

        // Add the verse to the store
        bible.add(&verses_js, None).await?;
        i = i  + 1;
    };
    
    // Waits for the transaction to complete
    transaction.done().await?;

    Ok(num_traits::cast(i).unwrap())
}