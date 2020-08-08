#![feature(proc_macro_hygiene, decl_macro)]


use std::collections::{HashMap, HashSet};

use std::{fs, io};
use std::path::Path;

use std::{process::Command};

use rocket::Request;
use rocket::response::content::Content;
use rocket::http::ContentType;

// use rocket_contrib::templates::Template;

// use tera::Context;


#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_include_tera;
#[macro_use] extern crate rocket_include_static_resources;
#[macro_use] extern crate lazy_static;

use rocket_include_tera::{TeraResponse// , TeraContextManager
};

use rocket_include_static_resources::StaticResponse;


// TODO: Replace default handler for following HTTP codes.
// 400, 401, 402, 403, 404, 405, 406, 407, 408, 409, 410, 411, 412, 413, 414, 415, 416, 417, 418, 421, 426, 428, 429, 431, 451, 500, 501, 503, and 510


fn first_machine_path_in_collectd_path() -> Option<String> {
    let data_source_base_path = fs::read_dir("/var/lib/collectd/rrd/").ok()?
        .next()?.ok()?
        .path()
        .to_str()?
        .to_string();

    Some(data_source_base_path)

}


lazy_static! {
   static ref DATA_SOURCE_BASE_PATH: String = first_machine_path_in_collectd_path().unwrap();
}


const RRDGRAPH_COLOR_THEME_DARK: &'static [&'static str] = &[
        "-c", "BACK#000000",
        "-c", "SHADEA#000000",
        "-c", "SHADEB#000000",
        "-c", "FONT#DDDDDD",
        "-c", "CANVAS#202020",
        "-c", "GRID#666666",
        "-c", "MGRID#AAAAAA",
        "-c", "FRAME#202020",
        "-c", "ARROW#FFFFFF",
];


#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}


#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}


#[get("/favicon.ico")]
fn favicon() -> StaticResponse {
    static_response!("favicon.ico")
}


#[get("/favicon-16.png")]
fn favicon_16_png() -> StaticResponse {
    static_response!("favicon-16.png")
}


fn available_rrd_data_sources() -> std::vec::Vec<String> {
    fs::read_dir(&*DATA_SOURCE_BASE_PATH).unwrap()
        .map(
            |res| res.map(
                |e|
                e.file_name().into_string().unwrap()
        ))
        .collect::<Result<Vec<_>, io::Error>>().unwrap()
}


fn rrd_files_in_data_source(data_source_name: &str) -> std::vec::Vec<String> {
    let data_source_path = Path::new(&*DATA_SOURCE_BASE_PATH).join(data_source_name);
    fs::read_dir(data_source_path).unwrap()
        .map(
            |res| res.map(
                |e|
                e.path().to_str().unwrap().to_string()
        ))
        .collect::<Result<Vec<_>, io::Error>>().unwrap()
}


fn rrdgraph_command(args: Vec<&str>) -> Vec<u8> {
    Command::new("rrdtool")
        .arg("graph")
        .arg("-")              // write to stdout
        .arg("--start").arg((-86400 * 2).to_string())
        // .arg("-u").arg("100")
        .arg("-l").arg("0")
        .arg("-w").arg("800") // width
        .arg("-h").arg("200")  // height
        .arg("-r")
        .args(args)
        .args(RRDGRAPH_COLOR_THEME_DARK)
        .output().expect("").stdout
}


#[get("/graph/cpu-0")]
// #[get("/")]
fn graph_cpu_0() ->  rocket::response::Content<std::vec::Vec<u8>> {
    let output = rrdgraph_command(vec![
        &["DEF:user=",   &*DATA_SOURCE_BASE_PATH, "/cpu-0/cpu-user.rrd:value:AVERAGE"].concat(),   "AREA:user#00ff00:\"Benutzer\"",
        &["DEF:nice=",   &*DATA_SOURCE_BASE_PATH, "/cpu-0/cpu-nice.rrd:value:AVERAGE"].concat(),   "AREA:nice#00ffff:\"Prioritaet\":STACK",
        &["DEF:system=", &*DATA_SOURCE_BASE_PATH, "/cpu-0/cpu-system.rrd:value:AVERAGE"].concat(), "AREA:system#ff0000:\"System\":STACK",
        &["DEF:iowait=", &*DATA_SOURCE_BASE_PATH, "/cpu-0/cpu-wait.rrd:value:AVERAGE"].concat(),   "AREA:iowait#0000ff:\"IO-Wait\":STACK"
    ]);
    let response = Content(ContentType::PNG, output);
    response
}


#[get("/graph/<data_source_name>/<rrd_filename>", rank = 2)]
// #[get("/")]
fn graph(
    data_source_name: String,
    rrd_filename:     String
) -> rocket::response::Content<std::vec::Vec<u8>>
{
    let rrd_abs_filesystem_filename = String::from(&*DATA_SOURCE_BASE_PATH) + "/" + &data_source_name + "/" + &rrd_filename;
    let output = rrdgraph_command(vec![
        &["DEF:user=", &rrd_abs_filesystem_filename, ":value:AVERAGE"].concat(),     &["AREA:user#00ff00:\"", "test", "\""].concat()
    ]);
    let response = Content(ContentType::PNG, output);
    response
}


#[get("/all_graphs")]
fn all_graphs() -> TeraResponse {
    let abs_path_part = String::from(&*DATA_SOURCE_BASE_PATH) + "/";
    let data_sources_with_files: HashMap<_, _> =
        available_rrd_data_sources()
        .into_iter()
        .map(|a| {
            let rrd_files: HashSet<_> = rrd_files_in_data_source(&a)
                .into_iter()
                .map(|abs_file| abs_file.replacen(&abs_path_part, "", 1))
                .collect();
            (a, rrd_files)
        })
        .collect();

    let mut context = tera::Context::new();

    context.insert("data_sources_with_files", &data_sources_with_files);

    tera_response!("all_graphs", &context)
}


#[get("/")]
fn index() -> TeraResponse {
    all_graphs()
}


fn main() {
    // rocket::ignite().register(catchers![internal_error, not_found]);
    // rocket::ignite().mount("/",            routes![graph]).launch();

    let dbg_msg = String::from("Data source directory is: ") + &*DATA_SOURCE_BASE_PATH;
    dbg!(dbg_msg);

    rocket::ignite()
        .mount("/",  routes![
            graph_cpu_0,
            graph,
            all_graphs,
            index,
            favicon,
            favicon_16_png
        ])
        .attach(StaticResponse::fairing(|resources| {
            static_resources_initialize!(
                resources,
                "favicon.ico",     "static/front-end/images/favicon.ico",
                "favicon-16.png",  "static/front-end/images/favicon-16.png",
                // "html-readme", "static/front-end/html/README.html",
            );
        }))
        .attach(TeraResponse::fairing(|tera| {
            tera_resources_initialize!(
                tera,
                "all_graphs",  "templates/all_graphs.html.tera"
            );
        }))
        .launch();
}
