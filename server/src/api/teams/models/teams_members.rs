use diesel::{
    Selectable,
    prelude::{Associations, Identifiable, Insertable, Queryable},
};

use crate::api::{
    teams::{TeamRole, models::teams::Team},
    users::User,
};

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Insertable)]
#[diesel(belongs_to(Team))]
#[diesel(belongs_to(User))]
#[diesel(table_name = crate::schema::teams_members)]
#[diesel(primary_key(team_id, user_id))]
pub struct TeamMember {
    pub team_id: String,
    pub user_id: String,
    pub role: TeamRole,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
