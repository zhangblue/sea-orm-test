use entity::class::Model;
use sea_orm::{
    DatabaseBackend, DatabaseConnection, EntityTrait, JoinType, QuerySelect, QueryTrait,
};

pub async fn query_class_example(database_connection: &DatabaseConnection) {
    println!("查询所有的班级(单表查询):");
    let all_class = query_all_class(database_connection).await;
    for class in all_class {
        println!("{:?}", class);
    }

    println!("查询所有的班级的课程信息(单表查询):");
    let class_course_vec = query_class_course(database_connection).await;
    for class_course in class_course_vec {
        println!("{:?}", class_course)
    }
}

/// 得到所有的班级
async fn query_all_class(database_connection: &DatabaseConnection) -> Vec<Model> {
    entity::class::Entity::find()
        .all(database_connection)
        .await
        .unwrap_or(Vec::new())
}

/// 根据班级得到当前班级所有的课程
async fn query_class_course(database_connection: &DatabaseConnection) -> Vec<ClassCourseModel> {
    let select_sql = entity::class::Entity::find()
        .select_only()
        .column(entity::class::Column::Id)
        .column(entity::class::Column::Class)
        .column(entity::class::Column::Grade)
        .column(entity::course::Column::CourseName)
        .join(
            JoinType::LeftJoin,
            entity::prelude::Class::belongs_to(entity::prelude::ClassCourse)
                .from(entity::class::Column::Id)
                .to(entity::class_course::Column::ClassId)
                .into(),
        )
        .join(
            JoinType::LeftJoin,
            entity::prelude::ClassCourse::belongs_to(entity::prelude::Course)
                .from(entity::class_course::Column::CourseId)
                .to(entity::course::Column::Id)
                .into(),
        );

    let sql = select_sql.build(DatabaseBackend::Postgres).to_string();

    println!("SQL: {}", sql);

    let class_course_result = select_sql
        .into_model::<ClassCourseModel>()
        .all(database_connection)
        .await;

    class_course_result.unwrap_or_else(|err| {
        eprintln!("查询失败:{}", err);
        Vec::new()
    })
}

#[derive(sea_orm::FromQueryResult, Debug)]
pub struct ClassCourseModel {
    id: i16,
    class: i32,
    grade: i32,
    course_name: String,
}
