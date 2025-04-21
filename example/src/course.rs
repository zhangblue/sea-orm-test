use entity::course::Model;
use sea_orm::{DatabaseConnection, EntityTrait};

/// 查询所有课程
pub async fn query_all_courses(database_connection: &DatabaseConnection) -> Vec<Model> {
    entity::course::Entity::find()
        .all(database_connection)
        .await
        .unwrap_or(Vec::new())
}
