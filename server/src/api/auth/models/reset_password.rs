use chrono::{Duration, Utc};
use diesel::{
    Selectable,
    prelude::{Identifiable, Insertable, Queryable},
};

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::password_reset_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(user_id))]
pub struct PasswordResetToken {
    pub user_id: String,
    pub token_hash: String,
    pub created_at: chrono::NaiveDateTime,
}
impl PasswordResetToken {
    pub fn is_expired(&self) -> bool {
        let now = Utc::now().naive_utc();
        let expiry_time = self.created_at + Duration::minutes(15);
        now > expiry_time
    }

    pub fn is_valid(&self) -> bool {
        !self.is_expired()
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::password_reset_tokens)]
pub struct AddPasswordResetToken {
    pub user_id: String,
    pub token_hash: String,
}
