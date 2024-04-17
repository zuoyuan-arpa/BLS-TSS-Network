use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NodeInfo {
    pub node_name: String,
    pub spec_version: String,
    pub node_version: String,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: String,
}

#[get("/eigen/node")]
pub async fn get_node_info() -> impl Responder {
    let node_info = NodeInfo {
        node_name: "EigenLayer-AVS".to_string(),
        spec_version: "v0.0.1".to_string(),
        node_version: "v1.0.0".to_string(),
    };
    HttpResponse::Ok().json(node_info)
}

#[get("/eigen/node/health")]
pub async fn get_node_health() -> impl Responder {
    // Implement node health check logic here
    HttpResponse::Ok()
}

#[get("/eigen/node/services")]
pub async fn get_node_services() -> impl Responder {
    let services = vec![
        ServiceInfo {
            id: "db-1".to_string(),
            name: "Database".to_string(),
            description: "Database description".to_string(),
            status: "Up".to_string(),
        },
        ServiceInfo {
            id: "idx-2".to_string(),
            name: "Indexer".to_string(),
            description: "Indexer description".to_string(),
            status: "Down".to_string(),
        },
    ];
    HttpResponse::Ok().json(services)
}