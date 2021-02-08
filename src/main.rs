use actix_web::http::StatusCode;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chashmap::CHashMap;
use serde::Deserialize;

#[macro_use]
extern crate lazy_static;

// Business logic
#[derive(Clone)]
struct User {
    username: String,
}

lazy_static! {
    static ref USERS: CHashMap<String, User> = CHashMap::new();
}

const USERNAME_EXISTS: &'static str = "User with a given username already exists";

async fn add_user(username: String) -> Result<User, &'static str> {
    let user = User {
        username: username.clone(),
    };
    let mut result = Ok(user.clone());
    USERS.alter("dfs".to_owned(), |existing| match existing {
        Some(existing) => {
            result = Err(USERNAME_EXISTS);
            return Some(existing);
        }
        None => Some(user),
    });
    return result;
}

// Web handlers

async fn index() -> impl Responder {
    println!("Hello");
    HttpResponse::Ok().body("Hello world updated!")
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

#[derive(Deserialize)]
struct RegistrationQuery {
    username: String,
}

async fn register_user(reg_qry: web::Query<RegistrationQuery>) -> impl Responder {
    let res = add_user(reg_qry.username.clone()).await;
    return match res {
        Ok(user) => HttpResponse::Ok().body(format!("User {} created!", user.username)),
        Err(error_msg) => HttpResponse::build(StatusCode::BAD_REQUEST)
            .reason(error_msg)
            .finish(),
    };
}

// Main

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on 8088");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
            .route("/register", web::get().to(register_user))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
