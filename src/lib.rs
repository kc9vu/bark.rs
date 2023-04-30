pub mod my_errors;
pub mod my_types;

pub mod app {
    use super::my_types::*;
    use hyper::{Body, Client, Method, Request};
    use hyper_tls::HttpsConnector;
    use std::{env, error::Error, fs::File};

    pub fn read_file_config(path: &str) -> Result<Config, Box<dyn Error>> {
        Ok(serde_json::from_reader(File::open(path)?)?)
    }

    pub fn execute_exe_config() -> Option<String> {
        let path = env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("bark-cli.json");
        return if path.exists() {
            Some(path.to_str().unwrap().to_string())
        } else {
            None
        };
    }

    pub async fn push(opt: &Opt) -> Result<Resp, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!(
            "https://{}:{}/{}",
            opt.host.as_ref().unwrap(),
            opt.port.as_ref().unwrap(),
            opt.device_key.as_ref().unwrap()
        );
        let data = opt.dumps();

        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        let request = Request::builder()
            .method(Method::POST)
            .uri(url)
            .header(
                "Content-Type",
                if let Some(true) = opt.encrypt.as_ref() {
                    "application/x-www-form-urlencoded"
                } else {
                    "application/json; charset=utf-8"
                },
            )
            .body(Body::from(data))
            .unwrap();
        let response = client.request(request).await.unwrap();

        let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        // println!("Body len: {}", body_bytes.len());

        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

        Ok(serde_json::from_str(&body_str)?)
    }
}
