use std::env;
use uuid::Uuid;
use anyhow::{Result, anyhow};
use scylla::{FromRow, IntoTypedRows, QueryResult, SessionBuilder, Session};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .map_err(|_| anyhow::anyhow!("DATABASE_URL environment variable not set"))?;

    // Initialize the session with your ScyllaDB cluster
    let session = SessionBuilder::new()
        .known_node(&database_url)
        .build()
        .await?;

    // Example: Creating a new user
    let new_user = User::new(
        "Dolly".to_string(),
        "Partition".to_string(),
        77);
    create_user(&session, &new_user).await?;

    // Example: Reading user data
    read_user_data(&session, &new_user.user_id).await?;

    // Example: Updating a user's age
    update_user(&session, &new_user.user_id, 78).await?;

    // Example: Deleting a user
    delete_user(&session, &new_user.user_id).await?;

    Ok(())
}

#[derive(Debug, FromRow)]
struct User {
    user_id: Uuid,
    age: i32,
    first_name: String,
    last_name: String,
}

impl User {
    fn new(first_name: String, last_name: String, age: i32) -> User {
        User {
            user_id: Uuid::new_v4(),
            first_name,
            last_name,
            age,
        }
    }
}

async fn create_user(session: &Session, user: &User) -> Result<QueryResult> {
    let query = "INSERT INTO demo.users (user_id, first_name, last_name, age) \
                      VALUES (?, ?, ?, ?)";
    session.query(query, (user.user_id, &user.first_name, &user.last_name, user.age)).await
        .map_err(|e| anyhow!("Failed to add user: {}", e))
}

async fn read_user_data(session: &Session, user_id: &Uuid) -> Result<()> {
    let query = "SELECT * FROM demo.users WHERE user_id = ? LIMIT 1";
    match session.query(query, (user_id,)).await {
        Ok(response) => {
            let rows = response.rows.unwrap_or_default();
            for row in rows.into_typed::<User>() {
                match row {
                    Ok(user) => println!("{:?}", user),
                    Err(e) => return Err(anyhow!("Failed to parse row: {}", e)),
                }
            }
        }
        Err(e) => return Err(anyhow!("Failed to perform read operation: {}", e)),
    }
    Ok(())
}

async fn update_user(session: &Session, user_id: &Uuid, new_age: i32) -> Result<QueryResult> {
    let query = "UPDATE demo.users SET age = ? WHERE user_id = ?";
    session.query(query, (new_age, user_id)).await
        .map_err(|e| anyhow!("Failed to update user: {}", e))
}

async fn delete_user(session: &Session, user_id: &Uuid) -> Result<QueryResult> {
    let query = "DELETE FROM demo.users WHERE user_id = ?";
    session.query(query, (user_id,)).await
        .map_err(|e| anyhow!("Failed to delete user: {}", e))
}
