use example::{class, student};
use sea_orm::{ConnectOptions, DatabaseConnection};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let db_connection = create_database_connection().await;

    // 查询学生相关的内容
    student::query_student_example(&db_connection).await;
    // 查询班级相关的内容
    class::query_class_example(&db_connection).await;
}

/// 创建数据库连接
async fn create_database_connection() -> DatabaseConnection {
    let mut opt = ConnectOptions::new("postgres://zhang:123456789@localhost/sea_orm_test_db");

    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8));

    sea_orm::Database::connect(opt)
        .await
        .expect("创建数据库连接失败!")
}
