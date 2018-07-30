use schema::todos;

#[derive(Serialize, Queryable)]
pub struct Todo {
    pub id: i32,
    pub task: String,
}

#[table_name = "todos"]
#[derive(Insertable)]
pub struct NewTodo {
    pub id: Option<i32>,
    pub task: String,
}
