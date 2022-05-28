use migration::{Migrator, MigratorTrait};
use entity::cake::Entity as Cake;
use entity::cake;
use sea_orm::ActiveModelTrait;
use sea_orm::EntityTrait;
use sea_orm::ActiveValue;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let maybe_connection = sea_orm::Database::connect("postgres://postgres:docker@localhost:5432/world").await;
    if maybe_connection.is_err() {
        panic!("No connection to database!");
    }
    let connection = maybe_connection.unwrap();
    let up_result = Migrator::up(&connection, None).await;
    if up_result.is_err() {
        panic!("Tables not setup!");
    }
    
    let cheesecake = cake::ActiveModel {
        id: ActiveValue::NotSet, // primary key is NotSet as this will be inferred by the ORM
        name: ActiveValue::Set("cheesecake".to_owned())
    };

    let mut cheesecake = cheesecake.save(&connection).await.unwrap();

    cheesecake.name = ActiveValue::Set("Vegan Cheesecake".to_string());

    let mut cheesecake = cheesecake.save(&connection).await.unwrap();

    println!("Vegan Cheesecake saved.");

    let vegan_id = cheesecake.id.unwrap();

    println!("The id of the vegan cheesecake is {:?}", vegan_id);

    let cheese = Cake::find_by_id(vegan_id).one(&connection).await.unwrap();

    println!("After loading the cheesecake, it looks like: {:?}", cheese.unwrap());
    // let down_result = Migrator::down(&connection, None).await;
    // if down_result.is_err() {
    //     panic!("Tables not removed!");
    // }
    //let mut ctx = tera::Context::new();
    Ok(())
}
