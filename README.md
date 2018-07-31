# Rocket Todo MVC

A web application build in [Rust](https://www.rust-lang.org/). It use [Rocket](https://rocket.rs) & [Diesel](http://diesel.rs/).
Default Rocket app does not have any boilerplate or scaffolding.

This sample just an application which allow you to CRUD todo.

- **List** todo: GET /todos 
- **Create** todo: POST /todos
- **Show** todo: GET /todos/<todo_id>
- **New** todo: GET /todos/new
- **Edit** todo: GET /todos/<todo_id>/edit
- **Update** todo: PUT /todos/<todo_id>
- **Delete** todo: DELETE /todos/<todo_id>

## Demo

[Demo on Heroku](https://rocky-mesa-79895.herokuapp.com/) using [Rust Buildpack](https://github.com/emk/heroku-buildpack-rust).

## Instalation

First you need to [install Rust](https://www.rust-lang.org/install.html). Then Clone the repository and go in the folder

~~~bash
$ git clone https://github.com/rezhajulio/RocketTodoMVC.git
$ cd RocketTodoMVC
~~~

According to [Rocket](https://rocket.rs), you need to use Nightly version of [Rust](https://www.rust-lang.org/)

~~~bash
$ rustup override set nightly
~~~

Then install [diesel_cli](https://github.com/diesel-rs/diesel/tree/master/diesel_cli) and run database migrations

~~~bash
$ cargo install diesel_cli --no-default-features --features postgres
$ diesel setup
$ diesel migration run
~~~

And now you can build project

~~~bash
$ cargo run
~~~
