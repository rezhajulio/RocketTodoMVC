use diesel::prelude::*;
use diesel::LimitDsl;
use diesel::LoadDsl;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::Template;
use schema::todos::dsl::*;

use database;
use form::todo_form;
use model::todo_model;

#[get("/")]
pub fn index() -> Template {
    let connection = database::establish_connection();
    let results = todos
        .limit(20)
        .load::<todo_model::Todo>(&connection)
        .expect("Error loading todos");

    Template::render("todo/index", &results)
}

#[get("/<todo_id>")]
pub fn show(todo_id: i32) -> Template {
    let connection = database::establish_connection();
    let results = todos
        .filter(id.eq(todo_id))
        .limit(1)
        .load::<todo_model::Todo>(&connection)
        .expect("Error loading todo");
    Template::render("todo/show", results.first())
}

#[get("/new")]
pub fn new() -> Template {
    Template::render("todo/new", &())
}

#[post("/", data = "<form_data>")]
pub fn create(form_data: Form<todo_form::Todo>) -> Redirect {
    use diesel;

    let connection = database::establish_connection();
    let recipe = todo_model::NewTodo {
        id: None,
        task: form_data.get().task.to_string(),
    };

    match diesel::insert(&recipe).into(todos).execute(&connection) {
        Ok(_) => Redirect::to("/todos"),
        Err(error) => panic!("There was a problem: {:?}", error),
    }
}

#[get("/<todo_id>/edit")]
pub fn edit(todo_id: i32) -> Template {
    let connection = database::establish_connection();
    let results = todos
        .filter(id.eq(todo_id))
        .limit(1)
        .load::<todo_model::Todo>(&connection)
        .expect("Error loading todos");
    Template::render("todo/edit", results.first())
}

#[put("/<todo_id>", data = "<form_data>")]
pub fn update(todo_id: i32, form_data: Form<todo_form::Todo>) -> Redirect {
    use diesel;

    let connection = database::establish_connection();

    let result = diesel::update(todos.find(todo_id))
        .set(task.eq(form_data.get().task.to_string()))
        .execute(&connection);

    match result {
        Ok(_) => Redirect::to(&format!("/todos/{}", todo_id)),
        Err(error) => panic!("There was a problem: {:?}", error),
    }
}

#[delete("/<todo_id>")]
pub fn delete(todo_id: i32) -> Redirect {
    use diesel;

    let connection = database::establish_connection();

    match diesel::delete(todos.find(todo_id)).execute(&connection) {
        Ok(_) => Redirect::to("/todos"),
        Err(error) => panic!("There was a problem: {:?}", error),
    }
}
