
// #[derive(Insertable)]
// #[table_name = "todos"]
#[derive(Debug, FromForm)]
pub struct Todo {
    pub task: String,
}
