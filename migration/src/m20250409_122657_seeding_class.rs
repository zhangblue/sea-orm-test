use sea_orm::{EntityTrait, Set};
use sea_orm_migration::prelude::*;
use crate::m20250408_093456_create_class;

#[derive(DeriveMigrationName)]
pub struct Migration;

/// 初始化时向class表中插入数据
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let connection = manager.get_connection();
        let seed_data_active_module_vec: Vec<entity::class::ActiveModel> = get_seeding_data_vec()
            .into_iter()
            .map(|x| entity::class::ActiveModel {
                id: Set(x.id),
                class: Set(x.class),
                grade: Set(x.grade),
            })
            .collect();

        entity::class::Entity::insert_many(seed_data_active_module_vec)
            .exec(connection)
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .truncate_table(Table::truncate().table(m20250408_093456_create_class::Class::Table).to_owned())
            .await
    }
}

fn get_seeding_data_vec() -> Vec<ClassSeedModule> {
    vec![
        ClassSeedModule {
            id: 0,
            class: 1,
            grade: 1,
        },
        ClassSeedModule {
            id: 1,
            class: 2,
            grade: 1,
        },
        ClassSeedModule {
            id: 2,
            class: 1,
            grade: 2,
        },
        ClassSeedModule {
            id: 3,
            class: 2,
            grade: 2,
        },
        ClassSeedModule {
            id: 4,
            class: 1,
            grade: 3,
        },
    ]
}

struct ClassSeedModule {
    id: i32,
    class: i16,
    grade: i16,
}
