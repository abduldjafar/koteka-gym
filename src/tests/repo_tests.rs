use crate::{
    config::{self, db::Sources},
    repo::{
        interface::DBInterface,
        model::{Id, User},
        surrealdb::Record,
    },
};

// Test to insert a user record into the database
#[tokio::test]
async fn test_insert_user_record() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize database source
    let mut surreal_db = config::db::DatabaseSource {
        db_type: config::db::DatabaseType::SurrealDB,
    };

    // Connect to the database
    let conn = surreal_db.connect().await?;

    // Create a new user
    let user = User {
        username: String::from("koteka"),
        user_type: String::from("gymnast"),
        email: String::from("xxxxxxx"),
        created_at: None,
        updated_at: None,
        password: String::from("asoigeboi"),
    };

    // Insert the user record
    let query_result: Option<Id> = conn.insert_record(String::from("user"), &user).await?;

    assert_ne!(None, query_result);

    Ok(())
}

// Test to select user records from the database
#[tokio::test]
async fn test_select_user_record() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize database source
    let mut surreal_db = config::db::DatabaseSource {
        db_type: config::db::DatabaseType::SurrealDB,
    };

    // Connect to the database
    let conn = surreal_db.connect().await?;

    // Select user records
    let result: Vec<User> = conn.select(String::from("user")).await?;

    // Assert records are found
    assert_ne!(0, result.len());

    Ok(())
}

// Test to delete a user record from the database
#[tokio::test]
async fn test_delete_user_record() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize database source
    let mut surreal_db = config::db::DatabaseSource {
        db_type: config::db::DatabaseType::SurrealDB,
    };

    // Connect to the database
    let conn = surreal_db.connect().await?;

    // Delete the user record
    let result = conn
        .delete(String::from("user:1xs8spkbmi1vfftq1q59"))
        .await?;

    // Assert deletion is successful
    assert_eq!(true, result);

    Ok(())
}

#[tokio::test]
async fn test_update_user_record() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize database source
    let mut surreal_db = config::db::DatabaseSource {
        db_type: config::db::DatabaseType::SurrealDB,
    };

    //  user data for update
    let user = User {
        username: String::from("koteka"),
        user_type: String::from("gymnast"),
        email: String::from("xxxxxx"),
        created_at: None,
        updated_at: None,
        password: String::from("asasas"),
    };

    // Connect to the database
    let conn = surreal_db.connect().await?;

    // Delete the user record
    let result = conn
        .update_record(
            String::from("user:6cxpicp1qxqr9o56mpab"),
            String::from("user"),
            &user,
        )
        .await?;

    // Assert deletion is successful
    assert_eq!(true, result);

    Ok(())
}

#[tokio::test]
async fn test_select_with_params() -> Result<(), Box<dyn std::error::Error>> {
    let mut surreal_db = config::db::DatabaseSource {
        db_type: config::db::DatabaseType::SurrealDB,
    };

    let conn = surreal_db.connect().await?;

    let result: Vec<User> = conn
        .select_where(
            String::from("user"),
            String::from("email = 'xxxxxxx'"),
            String::from(""),
        )
        .await?;

    assert_ne!(0, result.len());

    Ok(())
}