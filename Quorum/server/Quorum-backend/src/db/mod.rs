use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use std::error::Error;

pub mod schema;

pub type DB = Surreal<surrealdb::engine::remote::ws::Client>;

pub async fn init() -> Result<DB, Box<dyn Error>> {  
    let url = std::env::var("SURREAL_URL")
        .unwrap_or_else(|_| "ws://127.0.0.1:8000".to_string());
    let user = std::env::var("SURREAL_USER")?;
    let pass = std::env::var("SURREAL_PASS")?;
    let ns = std::env::var("SURREAL_NS")?;
    let db_name = std::env::var("SURREAL_DB")?;
    
    let db = Surreal::new::<Ws>(&url).await.map_err(|e| -> Box<dyn Error> {
        let error_msg = e.to_string();
        
        let hint = if error_msg.contains("10061") || error_msg.contains("actively refused") {
            "Hint: Connection refused. Is SurrealDB running?\n       Run: `docker compose up -d`"
        } else if error_msg.contains("No such host") || error_msg.contains("11001") {
            "Hint: Cannot resolve hostname. Check your SURREAL_URL in .env"
        } else if error_msg.contains("timeout") {
            "Hint: Connection timeout. Is the firewall blocking port 8000?"
        } else {
            "Hint: Check that SurrealDB is accessible at the URL in .env"
        };
        
        format!("Failed to connect to SurrealDB at {}\n{}\n\nError: {}", url, hint, e).into()
    })?;
    
    db.signin(Root {
        username: user,
        password: pass,
    })
    .await
    .map_err(|e| -> Box<dyn Error> {
        let hint = "Hint: Check SURREAL_USER and SURREAL_PASS in .env match docker-compose.yml";
        format!("Failed to authenticate\n{}\n\nError: {}", hint, e).into()
    })?;
    
    db.use_ns(&ns).use_db(&db_name).await.map_err(|e| -> Box<dyn Error> {
        let hint = "Hint: Verify SURREAL_NS and SURREAL_DB in .env exist. Run schema/initial.surql to create them.";
        format!("Failed to select database\n{}\n\nError: {}", hint, e).into()
    })?;
    
    Ok(db)
}