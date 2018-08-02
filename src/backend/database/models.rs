use super::schema::sessions;

#[derive(Queryable, Insertable)]
#[table_name="sessions"]
pub struct Session {
    pub id: String,
}