use env_logger::Env;
use tiny_file_server::{FileServer, url_to_file};

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    FileServer::http("127.0.0.1:9080")
        .expect("Server should be created")
        .with_url_to_file(|mut url, statics_path, default_file| {
            if !url.trim_start_matches('/').contains(['/', '.']) {
                url = ""
            }
            url_to_file(url, statics_path, default_file)
        })
        .run("target/web")
        .expect("Server should start");
}
