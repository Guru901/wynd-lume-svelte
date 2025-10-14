use std::env;

use lume::{
    database::{Database, error::DatabaseError},
    define_schema,
};
use wynd::wynd::Wynd;

define_schema! {
    Users {
        id: u64,
        name: String,
        email: String,
        password: String,
    }
}

#[tokio::main]
async fn main() -> Result<(), DatabaseError> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = Database::connect(&database_url).await?;

    db.register_table::<Users>().await?;

    db.insert(Users {
        id: 1,
        name: "John Doe".to_string(),
        email: "john.doe@example.com".to_string(),
        password: "password".to_string(),
    })
    .execute()
    .await?;

    let mut wynd = Wynd::new();

    wynd.on_connection(|conn| async move {
        println!("Client connected: {}", conn.id());

        conn.on_open(|handle| async move {
            handle.send_text("Hello from wynd").await.unwrap();
        })
        .await;

        conn.on_text(|event, handle| async move {
            if event.data == "get_users" {
                let users = get_users().await;
                handle
                    .send_text(format!("Users: {}", users.join(", ")).as_str())
                    .await
                    .unwrap();
            }
            handle
                .broadcast
                .text(format!("Someone said: {}", event.data).as_str())
                .await;
            handle
                .send_text(format!("You said: {}", event.data).as_str())
                .await
                .unwrap();
        });
    });

    wynd.listen(3000, || {
        println!("listening on http://localhost:3000");
    })
    .await
    .unwrap();

    Ok(())
}

async fn get_users() -> Vec<String> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = Database::connect(&database_url).await.unwrap();

    let users = db
        .query::<Users, SelectUsers>()
        .select(SelectUsers::selected().name())
        .execute()
        .await
        .unwrap();

    let user_names = users
        .iter()
        .map(|user| user.get(Users::name()).unwrap())
        .collect::<Vec<String>>();

    user_names
}
