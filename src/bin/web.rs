#[macro_use]
extern crate lazy_static;

use serde::Deserialize;

use tera::{Tera, Context};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use kris_kringle::conf;
use std::env;

async fn index() -> impl Responder {
    let mut context = Context::new();
    context.insert("name", "Gotham");
    let rendered = TERA.render("index.html.tera", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[derive(Deserialize)]
struct ParticipantsParams {
    num_participants: usize
}

async fn participants(params: web::Query<ParticipantsParams>) -> impl Responder {
    let mut context = Context::new();
    context.insert("num_participants",  &vec![0;params.num_participants]);
    let rendered = TERA.render("participants.html.tera", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}


async fn allocate_kks(req: String) -> impl Responder {
    let body = parse_body(&req);
    let conf = conf::KkConf::new(body).unwrap();
    let kris_kringles = kris_kringle::KrisKringles::from_config(conf);
    let mut context = Context::new();
    context.insert("pairs", &kris_kringles.get_pairs());
    let rendered = TERA.render("allocate.html.tera", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}

struct Param<'a> {
    key: &'a str,
    value: &'a str
}

fn parse_body(body: &String) -> Vec<conf::Participants> {
    let mut params = body.split('&').map(|x| {
        let mut split = x.split('=');
        Param {key: split.next().unwrap(), value: split.next().unwrap()}
    });

    let mut result = vec![];
    while let Some(name) = params.next() {
        let group = params.next().expect("Found name but no group");
        let participant = conf::Participants::new(
            name.value.to_owned(),
            group.value.parse().expect("Expected group to be a number"));
        result.push(participant)
    }
    result
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8088".to_string())
        .parse()
        .expect("PORT must be a number");

    println!("Starting server at http://localhost:{}", port);

    log::set_logger(|max_log_level| {
        max_log_level.set(::log::LogLevelFilter::Debug);
        Box::new(kris_kringle::kk_log::SimpleLogger)
    }).unwrap();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/participants", web::get().to(participants))
            .route("/allocate", web::post().to(allocate_kks))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

lazy_static! {
    pub static ref TERA: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".html.tera"]);
        tera
    };
}