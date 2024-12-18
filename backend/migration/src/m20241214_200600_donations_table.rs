use sea_orm_migration::{prelude::*, schema::*};

use crate::m20241214_112002_users_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Donations::Table)
                    .if_not_exists()
                    .col(pk_auto(Donations::Id).uuid())
                    .col(string(Donations::DonorId).uuid().not_null())
                    .col(string(Donations::Title).not_null())
                    .col(string(Donations::Description).text())
                    .col(string(Donations::Quantity).integer().not_null())
                    .col(string(Donations::Category))
                    .col(string(Donations::Status).not_null().default("available"))
                    .col(
                        string(Donations::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        string(Donations::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(string(Donations::PickupAddress).not_null())
                    .col(string(Donations::PickupCity).not_null())
                    .col(string(Donations::PickUpTime).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-donations-donor-id")
                            .from(Donations::Table, Donations::DonorId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Donations::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Donations {
    Table,
    Id,
    DonorId,
    Title,
    Description,
    Quantity,
    Category,
    Status,
    CreatedAt,
    UpdatedAt,
    PickupAddress, // New field
    PickupCity,
    PickUpTime,
}
