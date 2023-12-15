use serde_json::json;
use sysinfo::{CpuExt, System, SystemExt};
use warp::Filter;

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_origin("http://localhost:5173")
        .allow_methods(vec!["GET", "POST", "DELETE"]);

    let sysinfo = warp::path("sysinfo")
        .map(move || {
            let sys = System::new_all();
            let cpu_brand = sys.cpus().get(0).unwrap().brand().to_string();
            let cpu_frequency = sys.cpus().get(0).unwrap().frequency();
            let ram = sys.total_memory();
            let ram_used = sys.used_memory();
            let os = sys.long_os_version();

            json!({
              "cpu": {"brand": cpu_brand, "frequency": cpu_frequency },
              "ram": ram,
              "ram_used": ram_used,
              "uptime": sys.uptime(),
              "os": os,
            })
            .to_string()
        })
        .with(cors);

    warp::serve(sysinfo).run(([127, 0, 0, 1], 8181)).await;
}
