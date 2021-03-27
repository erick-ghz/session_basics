extern crate actix_session;

use actix_session::{CookieSession, Session};
use actix_web::{middleware::Logger, web, App, HttpRequest, HttpServer, Result, HttpResponse, Error};
use actix_http::{http};


/// simple index handler with session
async fn index(session: Session, req: HttpRequest) -> Result<&'static str> {
    println!("{:?}", req);

    // RequestSession trait is used for session access
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter").unwrap() {
        println!("SESSION value: {}", count);
        counter = count + 1;
        session.set("counter", counter).unwrap();
    } else {
        session.set("counter", counter).unwrap();
    }

    Ok("welcome!")
}

fn index3(session: Session) {
    /* let app = App::new().wrap(
        CookieSession::signed(&[0; 32])
            .domain("www.rust-lang.org")
            .name("actix_session")
            .path("/")
            .secure(true))
        .service(web::resource("/").to(|| HttpResponse::Ok())); */

}

async fn index2() -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::Ok()
        .header(http::header::CONTENT_TYPE, "application/json")
        .body("Hola")
    )
}


async fn index0(session: Session, req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    println!("{:?}", req);

    // RequestSession trait is used for session access
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        counter = count + 1;
        session.set("counter", counter)?;
    } else {
        session.set("counter", counter)?;
    }

    Ok(HttpResponse::Ok().body("Session"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    //env_logger::init();
    println!("Starting http server: 127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            // enable logger
            //.wrap(Logger::default())
            // cookie session middleware
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .service(web::resource("/").to(index0))
    })
    .bind("127.0.0.1:8082")?
    .run()
    .await
}



/*
#[actix_web::main]
async fn main() -> std::io::Result<()> {


    
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            //wrap(CookieSession::signed(&[0; 32]).secure(false))
            .service(web::resource("/session")
                .route(web::get().to(index2))
            )
            .route("/hey", web::get().to(index2))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}*/



/*
#[actix_web::main]
async fn main() {//-> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
  
    println!("Starting http server: 127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            // enable logger
            //.wrap(Logger::default())
            // cookie session middleware
          //  .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .service(web::resource("/").to(index2))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
    .unwrap()
}*/