use sea_orm::prelude::DateTime;
use sea_orm::{
    DatabaseBackend, DatabaseConnection, EntityTrait, FromQueryResult, JoinType, QuerySelect,
    QueryTrait, RelationTrait,
};

// 查询学生相关信息
pub async fn query_student_example(database_connection: &DatabaseConnection) {
    // 查询学生与班级的信息
    println!("使用外键join查询学生与班级的信息(多对一关系):");
    let student_class = query_student_class(database_connection).await;
    for student in student_class {
        println!("{:?}", student);
    }
    println!("手动join查询学生与班级的信息(多对一关系):");
    let student_class = query_student_class_customer_join(database_connection).await;
    for student in student_class {
        println!("{:?}", student);
    }
}

/// 学生与班级的数据
async fn query_student_class(database_connection: &DatabaseConnection) -> Vec<StudentClass> {
    let select_sql = entity::student::Entity::find()
        .select_only()
        .column(entity::student::Column::Id)
        .column(entity::student::Column::Name)
        .column(entity::student::Column::Age)
        .column(entity::student::Column::Gender)
        .column(entity::student::Column::ClassId)
        .column(entity::class::Column::Class)
        .column(entity::class::Column::Grade)
        .column(entity::student::Column::EnrollmentDate)
        .join(JoinType::LeftJoin, entity::student::Relation::Class.def());

    println!(
        "sql: {}",
        select_sql.build(DatabaseBackend::Postgres).to_string()
    );

    let student_class_result = select_sql
        .into_model::<StudentClass>()
        .all(database_connection)
        .await;
    student_class_result.unwrap_or_else(|err| {
        eprintln!("{}", err);
        Vec::new()
    })
}

/// 使用手动join的方式进行查询。
async fn query_student_class_customer_join(
    database_connection: &DatabaseConnection,
) -> Vec<StudentClass> {
    let select_sql = entity::student::Entity::find()
        .select_only()
        .column(entity::student::Column::Id)
        .column(entity::student::Column::Name)
        .column(entity::student::Column::Age)
        .column(entity::student::Column::Gender)
        .column(entity::student::Column::ClassId)
        .column(entity::class::Column::Class)
        .column(entity::class::Column::Grade)
        .join(
            JoinType::LeftJoin,
            entity::prelude::Student::belongs_to(entity::prelude::Class)
                .from(entity::student::Column::ClassId)
                .to(entity::class::Column::Id)
                .into(),
        );

    println!(
        "sql: {}",
        select_sql.build(DatabaseBackend::Postgres).to_string()
    );

    let student_class_result = select_sql
        .into_model::<StudentClass>()
        .all(database_connection)
        .await;
    student_class_result.unwrap_or_else(|err| {
        eprintln!("{}", err);
        Vec::new()
    })
}

/// 返回结构
#[derive(Debug, FromQueryResult)]
struct StudentClass {
    id: i32,                   // 学生id
    name: String,              // 学生姓名
    age: i16,                  // 学生年龄
    gender: i16,               // 学生性别
    enrollment_date: DateTime, // 入学时间
    class_id: i32,             // 班级id
    class: i16,                // 班级
    grade: i16,                // 年级
}
