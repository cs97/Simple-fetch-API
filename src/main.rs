


use actix_web::{get, put, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    let data = std::fs::read("www/index.html").expect("Cannot read index file");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(data)
}

#[get("/library.js")]
async fn library() -> impl Responder {
    let data = std::fs::read("www/library.js").expect("Cannot read index file");
    HttpResponse::Ok()
        .content_type("text/javascript")
        .body(data)
}

#[get("/app.js")]
async fn app() -> impl Responder {
    let data = std::fs::read("www/app.js").expect("Cannot read index file");
    HttpResponse::Ok()
        .content_type("text/javascript")
        .body(data)
}



#[put("/submit")]
async fn submit(req_body: String) -> impl Responder {
	println!("{}", req_body);
    HttpResponse::Ok()
}



async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
			.service(index)
			.service(submit)
			.service(library)
			.service(app)

            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
