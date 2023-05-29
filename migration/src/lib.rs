pub use sea_orm_migration::prelude::*;

mod m20230309_180650_cash_flows;
mod m20230309_214510_entries;
mod m20230528_204409_wallets;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230309_180650_cash_flows::Migration),
            Box::new(m20230309_214510_entries::Migration),
            Box::new(m20230528_204409_wallets::Migration),
        ]
    }
}
