pub use sea_orm_migration::prelude::*;

mod m20250408_093421_create_student;
mod m20250408_093456_create_class;
mod m20250408_093514_create_course;
mod m20250409_074219_create_class_course;
mod m20250409_093741_seeding_course;
mod m20250409_122657_seeding_class;
mod m20250421_024423_seeding_student;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250408_093456_create_class::Migration),
            Box::new(m20250408_093421_create_student::Migration),
            Box::new(m20250408_093514_create_course::Migration),
            Box::new(m20250409_074219_create_class_course::Migration),
            Box::new(m20250409_093741_seeding_course::Migration),
            Box::new(m20250409_122657_seeding_class::Migration),
            Box::new(m20250421_024423_seeding_student::Migration),
        ]
    }
}
