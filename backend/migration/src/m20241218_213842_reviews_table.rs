use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20241214_112002_users_table::Users, m20241214_200600_donations_table::Donations};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Reviews::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Reviews::Id).uuid().not_null().primary_key())
                    .col(string(Reviews::DonationId).uuid().not_null())
                    .col(string(Reviews::ReviewerId).uuid().not_null())
                    .col(
                        ColumnDef::new(Reviews::Rating)
                            .integer()
                            .not_null()
                            .check(Expr::col(Reviews::Rating).between(1, 5)),
                    )
                    .col(ColumnDef::new(Reviews::Comment).text())
                    .col(
                        ColumnDef::new(Reviews::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-reviews-donation-id")
                            .from(Reviews::Table, Reviews::DonationId)
                            .to(Donations::Table, Donations::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-reviews-reviewer-id")
                            .from(Reviews::Table, Reviews::ReviewerId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Reviews::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Reviews {
    Table,
    Id,
    DonationId,
    ReviewerId,
    Rating,
    Comment,
    CreatedAt,
}
