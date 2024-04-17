use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use arpa_stats::{get_node_health, get_node_info, get_node_services};


#[get("/eigen/node/services/{service_id}/health")]
async fn get_service_health(path: web::Path<(String)>) -> impl Responder {
    // Implement service health check logic here, using the service_id
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_node_info)
            .service(get_node_health)
            .service(get_node_services)
            .service(get_service_health)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}