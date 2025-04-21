use crate::m20250409_074219_create_class_course;
use sea_orm::EntityTrait;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

/// 想course表中插入数据
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let connection = manager.get_connection();

        let course_vec = get_seed_data_vec();

        let course_active_model_vec: Vec<entity::course::ActiveModel> = course_vec
            .into_iter()
            .map(|x| entity::course::ActiveModel {
                id: sea_orm::Set(x.id),
                course_name: sea_orm::Set(x.course_name),
            })
            .collect();

        entity::course::Entity::insert_many(course_active_model_vec)
            .exec(connection)
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .truncate_table(
                Table::truncate()
                    .table(m20250409_074219_create_class_course::ClassCourse::Table)
                    .to_owned(),
            )
            .await
    }
}

fn get_seed_data_vec() -> Vec<CourseSeedModel> {
    vec![
        CourseSeedModel {
            id: 0,
            course_name: "语文".to_string(),
        },
        CourseSeedModel {
            id: 1,
            course_name: "数学".to_string(),
        },
        CourseSeedModel {
            id: 2,
            course_name: "英语".to_string(),
        },
        CourseSeedModel {
            id: 3,
            course_name: "物理".to_string(),
        },
        CourseSeedModel {
            id: 4,
            course_name: "化学".to_string(),
        },
        CourseSeedModel {
            id: 5,
            course_name: "生物".to_string(),
        },
        CourseSeedModel {
            id: 6,
            course_name: "历史".to_string(),
        },
        CourseSeedModel {
            id: 7,
            course_name: "地理".to_string(),
        },
        CourseSeedModel {
            id: 8,
            course_name: "政治".to_string(),
        },
        CourseSeedModel {
            id: 9,
            course_name: "美术".to_string(),
        },
        CourseSeedModel {
            id: 10,
            course_name: "音乐".to_string(),
        },
    ]
}

struct CourseSeedModel {
    id: i32,
    course_name: String,
}
