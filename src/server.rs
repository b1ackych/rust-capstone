use rust_capstone::schema::*;  

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use diesel::{Insertable, Queryable};
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "vaults"]  
struct Vault {
    id: i32,
    user_id: String,
    encrypted_key: String,
    encrypted_data: String,
}

fn store_vault_data(vault_data: &Vault) -> Result<(), diesel::result::Error> {
    let connection = establish_connection()?;
    diesel::insert_into(vaults::table)
        .values(vault_data)
        .execute(&connection)
        .map(|_| ()) 
}

fn load_vault_data() -> Result<Vec<Vault>, diesel::result::Error> {
    let connection = establish_connection()?;
    let results = vaults::table.load::<Vault>(&connection)?;  
    Ok(results)
}

fn establish_connection() -> Result<PooledConnection<ConnectionManager<PgConnection>>, diesel::result::Error> {
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .map_err(|_| diesel::result::Error::NotFound)?;

    pool.get().map_err(|_| diesel::result::Error::NotFound)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/vault/store", web::post().to(store_data))
            .route("/vault/load", web::get().to(load_data))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn store_data(vault_data: web::Json<Vault>) -> impl Responder {
    match store_vault_data(&vault_data.into_inner()) {
        Ok(_) => HttpResponse::Ok().body("Data stored successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to store data: {}", e)),
    }
}

async fn load_data() -> impl Responder {
    match load_vault_data() {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to load data: {}", e)),
    }
}