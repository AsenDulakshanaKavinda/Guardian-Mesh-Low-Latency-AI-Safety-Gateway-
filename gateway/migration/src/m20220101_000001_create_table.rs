use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum User {
    Table,
    UserID,
    Username,
    Email,
    Password,
    CreatedAt,
}

#[derive(DeriveIden)]
enum BadPrompt {
    Table,
    PromptID,
    UserID,
    Prompt,
    Type,
    ReqestedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::UserID).string().not_null().primary_key())
                    .col(ColumnDef::new(User::Username).string().not_null())
                    .col(ColumnDef::new(User::Email).string().unique_key().not_null())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::CreatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(BadPrompt::Table)
                    .if_not_exists()
                    // Set PromptID as the Primary Key
                    .col(ColumnDef::new(BadPrompt::PromptID).string().not_null().primary_key())
                    .col(ColumnDef::new(BadPrompt::UserID).string().not_null())
                    .col(ColumnDef::new(BadPrompt::Prompt).string().not_null())
                    .col(ColumnDef::new(BadPrompt::Type).string().not_null())
                    .col(ColumnDef::new(BadPrompt::ReqestedAt).date_time().not_null())
                    // Set up the Foreign Key relationship to User::UserID
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-bad_prompt-user_id")
                            .from(BadPrompt::Table, BadPrompt::UserID)
                            .to(User::Table, User::UserID)
                            .on_delete(ForeignKeyAction::Cascade) // Deletes prompts if the user is deleted
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(BadPrompt::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}


