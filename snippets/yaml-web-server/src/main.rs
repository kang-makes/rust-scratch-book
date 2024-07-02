use axum::{routing::get, Router};
use chart::{merger, spec::Chart};
use glob::glob;

pub mod chart;

const CHART_FOLDER: &str = "charts";
const CHART_DESCRIPTOR_FILE: &str = "Chart.yaml";
const OVERRIDE_FOLDER: &str = "local";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let router = Router::new().route("/", get(say_hello_text));
    //let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    //axum::serve(listener, router).await.unwrap();

    Ok(())
}

async fn merge_charts() -> Vec<Chart> {
    let paths = glob(format!("./{CHART_FOLDER}/*/{CHART_DESCRIPTOR_FILE}").as_str())
        .expect("Failed to read glob pattern");
    let mut charts: Vec<Chart> = Vec::new();

    for path in paths {
        match path {
            Ok(path_buf) => {
                charts.push(merger::chart_from_file(OVERRIDE_FOLDER, path_buf).unwrap())
            }
            Err(err) => println!("error reading paths: {}", err),
        }
    }

    charts
}
