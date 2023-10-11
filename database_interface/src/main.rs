extern crate dotenvy;

use crate::db_lib::db_lib::*;
use dotenvy_macro::*;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{Error, MySql, Pool};

pub mod db_lib;

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    // Invoke .env file for compile time database checks
    dotenv!("DATABASE_URL");

    let pool = MySqlPoolOptions::new()
        .max_connections(7)
        .connect("mysql://root:root@localhost/medical_db")
        .await?;

    let rows = get_medical_report_providers(&pool).await?;
    println!("{}", rows[0]);
    Ok(())
}

async fn get_medical_report_providers(db_pool: &Pool<MySql>) -> Result<Vec<MedicalReportProvider>, Error> {
    let providers = sqlx::query_as!(
        MedicalReportProvider,
        "SELECT * FROM medical_report_provider"
    )
    .fetch_all(db_pool)
    .await?;

    Ok(providers)
}
