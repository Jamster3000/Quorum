use std::error::Error;

pub async fn init(db: &crate::db::DB) -> Result<(), Box<dyn Error>> {
    let schema = include_str!("../../../schema/initial.surql");

    db.query(schema).await?.check()?;

    Ok(())
}
