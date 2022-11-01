use std::path::{Path, PathBuf};

use reqwest::{Response, StatusCode};
use rocket::fs::NamedFile;
use rocket::{
    self,
    Config,
    response::status,
    serde::json::Json,
    serde::Deserialize,
    serde::json::serde_json::Deserializer,
};

use rocket_dyn_templates::Template;
use serde::Serialize;


use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use std::fs::OpenOptions;
use std::io::prelude::*;

#[derive(Debug, Serialize)]
struct Route {
    key: String,
    url: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct Create {
    url: String,
}

#[rocket::get("/")]
fn index() -> Template {

    Template::render("index", rocket_dyn_templates::context!{ webhooks: get_routes()})
}

#[rocket::post("/<webhook_key>", data = "<data>")]
async fn handle_webhook(webhook_key: String, data: String) -> Result<status::Accepted<String>, status::BadRequest<String>> {
    println!("got key: {}", webhook_key);
    
    let url = match get_route_from_key(webhook_key) {
        Ok(n) => n,
        Err(_) => {
            return Err(status::BadRequest(Some("Invalid Webhook Key".to_string())));
        }
    };

    println!("got url: {}", url);

    let client = reqwest::Client::new();
    let res = client.post(url)
        .body(data)
        .send()
        .await
        .unwrap();

    Ok(status::Accepted(Some(format!("{}", res.status()))))
}

#[rocket::post("/hook", data = "<data>")]
async fn create_webhook(data: Json<Create>) -> Result<status::Accepted<String>, status::BadRequest<String>> {
    println!("got {}", data.url);

    let hash = hash_int(get_num_routes()+1);

    println!("hash: {}", hash);

    write_route(Route {key: hash.clone(), url: data.url.clone()});

    Ok(status::Accepted(Some(format!("{}", hash.clone()))))
}

#[rocket::get("/static/<file>")]
async fn get_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).await.ok()
}

fn get_route_from_key(webhook_key: String) -> Result<String, u8> {
    for route in get_routes() {
        if route.key == webhook_key {
            return Ok(route.url);
        }
    }
    Err(0)
}

fn hash_int(val: usize) -> String{
    let mut hasher = DefaultHasher::new();
    val.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

fn write_route(route: Route) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("routes.txt")
        .unwrap();
    
    writeln!(file, "{}, {}", route.key, route.url);
}

fn get_routes() -> Vec<Route> {
    let mut file = OpenOptions::new()
        .read(true)
        .open("routes.txt")
        .unwrap();

    let mut contenets = String::new();

    let mut routes: Vec<Route> = vec![];

    file.read_to_string(&mut contenets);
    for line in contenets.lines() {
        let mut v= line.split(", ");
        let r: Route = Route {
            key: v.next().unwrap().to_string(),
            url: v.next().unwrap().to_string(),
        };

        routes.push(r);
        
    }

    return routes;

}

fn get_num_routes() -> usize {
    let mut file = OpenOptions::new()
        .read(true)
        .open("routes.txt")
        .unwrap();

    let mut contenets = String::new();
    file.read_to_string(&mut contenets);
    contenets.lines().count()
}

pub fn start_api() {
    rocket::tokio::runtime::Builder::new_multi_thread()
        .worker_threads(Config::from(Config::figment()).workers)
        // NOTE: graceful shutdown depends on the "rocket-worker" prefix.
        .thread_name("rocket-worker-thread")
        .enable_all()
        .build()
        .expect("create tokio runtime")
        .block_on(async move {
            let _ = rocket::build()
            .mount("/", rocket::routes![index, handle_webhook, create_webhook, get_file])
            .attach(Template::fairing())
            //.manage()
            .launch()
            .await;
        });
}