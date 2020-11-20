use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Request {
    parameter_1: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    data: Vec<String>,
    metadata: Option<String>,
}

#[get("/")]
async fn read_something(web::Query(params): web::Query<Request>) -> impl Responder {
    // Do something useful here

    log::debug!("{:?}", params);
    web::Json(Response {
        data: vec![String::from("Data")],
        metadata: Some(String::from("Metadata")),
    })
}

#[post("/")]
async fn create_something(web::Query(params): web::Query<Request>) -> impl Responder {
    // Do something useful here

    log::debug!("{:?}", params);
    web::Json(Response {
        data: vec![String::from("Data")],
        metadata: Some(String::from("Metadata")),
    })
}

#[delete("/")]
async fn delete_something(web::Query(params): web::Query<Request>) -> impl Responder {
    // Do something useful here

    log::debug!("{:?}", params);
    web::Json(Response {
        data: vec![String::from("Data")],
        metadata: Some(String::from("Metadata")),
    })
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
pub async fn main(host: std::net::IpAddr, port: u16) -> std::io::Result<()> {
    //let data = web::Data::new();

    HttpServer::new(move || {
        App::new()
            //.app_data(data.clone())
            .service(read_something)
            .service(create_something)
            .service(delete_something)
            .service(echo)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

// Note: Use the alternative below if actix must run in a tokio async function
// use actix_rt;
//pub async fn run_in_tokio_main(host: std::net::IpAddr, port: u16) -> std::io::Result<()> {
//    //let data = web::Data::new(options);
//    let local = tokio::task::LocalSet::new();
//    let sys = actix_rt::System::run_in_tokio("server", &local);
//    let server_res = HttpServer::new(move || {
//        App::new()
//            //.app_data(data.clone())
//            .service(read_something)
//            .service(create_something)
//            .service(delete_something)
//            .service(echo)
//    })
//    .bind(format!("{}:{}", host, port))?
//    .run()
//    .await?;
//    sys.await?;
//    Ok(server_res)
//}
