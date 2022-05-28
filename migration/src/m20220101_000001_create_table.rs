use sea_orm_migration::{prelude::*, sea_orm::{DatabaseConnection, DatabaseBackend, Schema}};
use entity::cake::*;
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let builder: DatabaseBackend = manager.get_database_backend();
        let schema = Schema::new(builder);
        match manager.create_table(schema.create_table_from_entity(Entity)).await {
            Ok(_) => println!("(up) Table created"),
            Err(error) => panic!("(up) Table not created due to error {:?}", error)
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        match manager
            .drop_table(Table::drop().table(Entity).to_owned())
            .await {
                Ok(_) => println!("(up) Table destroyed"),
                Err(error) => panic!("(up) Table not destroyed due to error {:?}", error)
            }
            Ok(())
    }
}
