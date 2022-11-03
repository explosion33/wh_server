use std::path::{Path, PathBuf};

use reqwest::{Response, StatusCode};
use rocket::{
    self,
    Config,
    response::status,
    serde::json::Json,
    serde::Deserialize,
    serde::Serialize,
    fs::NamedFile,
};

use rocket_dyn_templates::Template;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use std::fs::OpenOptions;
use std::io::{prelude::*, SeekFrom};

use crate::passwords::{hash_new, hash_old};

#[derive(Debug, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
struct Route {
    key: String,
    url: String,
    username: String,
    hash: String,
    salt: String
}

#[derive(Debug, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
struct RouteVisible {
    key: String,
    url: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct User {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct Create {
    url: String,
    username: String,
    password: String,
}


#[derive(Debug, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct Delete {
    key: String,
    username: String,
    password: String,
}


// ROUTES

#[rocket::get("/")]
fn index() -> Template {

    Template::render("index", rocket_dyn_templates::context!{})
}

#[rocket::post("/user_hooks", data = "<data>")]
fn get_user_webhooks(data: Json<User>) -> Json<Vec<RouteVisible>> {
    let mut user_routes: Vec<RouteVisible> = vec![];

    let mut is_user: bool = false;
    for route in get_routes() {
        if route.username == data.username && (is_user ||hash_old(data.password.clone(), route.salt.clone()).unwrap() == route.hash) {
            println!("{}, {}", route.username, data.username);
            let vr: RouteVisible = RouteVisible {key: route.key, url: route.url};
            user_routes.push(vr);
            is_user = true;
        }
    }

    return Json(user_routes);
}

#[rocket::post("/hook/<webhook_key>", data = "<data>")]
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
    let res = match client.post(url)
        .body(data)
        .send()
        .await
        {
            Ok(n) => n,
            Err(n) => {
                return Err(status::BadRequest(Some(format!("Error creating request\n{}", n))));
            }
        };

    Ok(status::Accepted(Some(format!("{}", res.status()))))
}

#[rocket::post("/hook", data = "<data>")]
fn create_webhook(data: Json<Create>) -> Result<status::Accepted<String>, status::BadRequest<String>> {
    println!("got {}", data.url);

    let key = match get_next_key() {
        Ok(n) => n,
        Err(_) => {
            return Err(status::BadRequest(Some("Internal Error".to_string())));
        }
    };

    println!("key: {}", key);

    let mut hash: String = String::new();
    let mut salt: String = String::new();
    let mut is_user = false;
    
    for route in get_routes() {
        if route.username == data.username {
            is_user = true;
            hash = hash_old(data.password.clone(), route.salt.clone()).unwrap();
            salt = route.salt;

            if hash != route.hash {
                return Err(status::BadRequest(Some("Invalid Password".to_string())));
            }
            break;
        }
    }

    if !is_user {
        (hash, salt) = hash_new(data.password.clone()).unwrap();
    }

    write_route(Route {key: key.clone(), url: data.url.clone(), username: data.username.clone(), hash, salt});

    Ok(status::Accepted(Some(format!("{}", key.clone()))))
}

#[rocket::post("/delete", data = "<data>")]
fn delete_webhook(data: Json<Delete>) -> Result<status::Accepted<String>, status::BadRequest<String>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("routes.txt")
        .unwrap();

    let mut output = String::new();

    let mut contenets = String::new();
    match file.read_to_string(&mut contenets) {
        Ok(_) => {},
        Err(n) => {
            println!("{}", n);
            return Err(status::BadRequest(Some("IO Error".to_string())));
        },
    };
    
    let mut found: bool = false;

    for line in contenets.lines() {
        let mut v= line.split(", ");

        let r: Route = Route {
            key: v.next().unwrap().to_string(),
            url: v.next().unwrap().to_string(),
            hash: v.next().unwrap().to_string(),
            salt: v.next().unwrap().to_string(),
            username: v.next().unwrap().to_string(),
        };

        if r.username == data.username && r.key == data.key {
            if !found && hash_old(data.password.clone(), r.salt.clone()).unwrap() == r.hash {
                println!("{}", r.key);
                found = true;
            }
            else {
                return Err(status::BadRequest(Some("Invalid Password".to_string())));
            }
            
            
        } 
        else {
            output += line;
            output += "\n";
        }
    }

    if !found {
        return Err(status::BadRequest(Some("Key not found".to_string())));
    }

    println!("{}", output);

    match file.set_len(0) {
        Ok(_) => {},
        Err(n) => {
            println!("{}", n);
            return Err(status::BadRequest(Some("IO Error".to_string())));
        },
    };
    match file.seek(SeekFrom::Start(0)) {
        Ok(_) => {},
        Err(n) => {
            println!("{}", n);
            return Err(status::BadRequest(Some("IO Error".to_string())));
        },
    };
    match write!(file, "{}", output) {
        Ok(_) => {},
        Err(n) => {
            println!("{}", n);
            return Err(status::BadRequest(Some("IO Error".to_string())));
        },
    };

    Ok(status::Accepted(Some(format!(""))))

}



#[rocket::get("/static/<file>")]
async fn get_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).await.ok()
}

// HELPER FUNCTIONS

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
    
    writeln!(file, "{}, {}, {}, {}, {}", route.key, route.url, route.hash, route.salt, route.username);
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
            hash: v.next().unwrap().to_string(),
            salt: v.next().unwrap().to_string(),
            username: v.next().unwrap().to_string(),
        };

        routes.push(r);
        
    }

    return routes;

}

fn get_next_key() -> Result<String, u8> {
    let mut file = OpenOptions::new()
        .read(true)
        .open("routes.txt")
        .unwrap();

    let mut contenets = String::new();
    match file.read_to_string(&mut contenets) {
        Ok(_) => {},
        Err(_) => {return Err(0)},
    };
    let key = match match contenets
        .lines()
        .last() {
            Some(n) => n,
            None => {
                return Ok(hash_int(1));
            },
        }
        .split(", ")
        .next() {
            Some(n) => n,
            None => {return Err(2)},
        };
    
    return Ok(hash_int(match usize::from_str_radix(key, 16) {
        Ok(n) => n,
        Err(_) => {return Err(3)},
    }));
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
            .mount("/", rocket::routes![index, handle_webhook, create_webhook, get_file, get_user_webhooks, delete_webhook])
            .attach(Template::fairing())
            //.manage()
            .launch()
            .await;
        });
}