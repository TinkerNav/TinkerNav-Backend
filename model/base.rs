#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "testing"]
pub struct Testing {
    pub id: i32,
    pub created: String,
    pub text: String,
    pub deleted: bool,
}