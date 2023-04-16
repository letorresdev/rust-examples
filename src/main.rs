
use reqwest;
use aws_sdk_dynamodb::{Client};
use serde_json;
use serde_json::{Result, Value};
use aws_config;
mod structure;
mod api;
use structure::structures::Data;
use api::futbolapi::get_data_futbol_api;


use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};

// This struct represents state
struct AppState {
    app_name: String,
    client: Client,
}

#[get("/tables_names")]
async fn index(data: web::Data<AppState>) -> String {
    // Get the tables 
    let res = &data.client.list_tables().send().await.unwrap();
    println!(" Table : {:?}", res.table_names);
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") ;
    format!("Hello {:?}!", res.table_names) // <- response with app_name
}


#[get("/futbol_data")]
async fn futbol_data() -> impl Responder {

    const ID : &str = "201";

    let data = get_data_futbol_api(ID).await.unwrap();
    let parsed_json: Value = serde_json::from_str(&data).unwrap(); // let parsed_json: Result<Data> or Result<Value>  = serde_json::from_str(&data);
    let response: Data = serde_json::from_value(parsed_json).unwrap();

    // This is other way to parse the data with match
    // let response: Data = match parsed_json {
    //     Ok(data) => data,
    //     Err(e) => panic!("Error parsing JSON: {}", e),
    // };

    HttpResponse::Ok().json(response.get_match_info())
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    // dbg!(res.table_names);

    HttpServer::new( move|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name : String::from("Actix Web Doctor"),
                client: client.clone(),
            }))
            .service(index)
            .service(futbol_data)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}