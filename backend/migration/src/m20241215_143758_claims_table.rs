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
                    .table(Claims::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Claims::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(string(Claims::DonationId).uuid().not_null())
                    .col(string(Claims::RecipientId).uuid().not_null())
                    .col(string(Claims::Status).not_null())
                    .col(
                        ColumnDef::new(Claims::ClaimedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-claims-donation-id")
                            .from(Claims::Table, Claims::DonationId)
                            .to(Donations::Table, Donations::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-claims-recipient-id")
                            .from(Claims::Table, Claims::RecipientId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Claims::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Claims {
    Table,
    Id,
    DonationId,
    RecipientId,
    Status,
    ClaimedAt,
}
