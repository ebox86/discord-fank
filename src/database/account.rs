use sea_orm::DatabaseConnection;
use sqlx::PgPool;
use super::Account;
use crate::database::Connection;

// pub async fn get_account_by_id(state: &DatabaseConnection, user_id: i64) -> Result<Vec<Account>, sqlx::Error> {
//     let account: Vec<Account> =
//         sqlx::query_as("SELECT * FROM account WHERE user_id = $1")
//             .bind(user_id)
//             .fetch_all(state)
//             .await.unwrap();
//     return if account.get(0).is_some(){(account.get(0).unwrap().xp, account.get(0).unwrap().level)};
//     Ok(account)
// }