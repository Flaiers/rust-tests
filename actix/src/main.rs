use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};


#[get("/")]
async fn get() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/")]
async fn post(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get)
            .service(post)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
