use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

pub struct Users {
    pub id: i32,
    pub name: String,
    pub dob: String,
    pub username: String,
    pub password: String,
}
