pub mod my_errors;
pub mod my_types;

pub mod app {
    use std::{
        env,
        error::Error,
        fs::File,
        io::{Read, Write},
        net::TcpStream,
        sync::Arc,
    };

    use rustls::{OwnedTrustAnchor, RootCertStore};

    use super::my_types::*;

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

    pub fn new_push(opt: &Opt) -> Resp {
        let host = opt.host.as_ref().unwrap().as_str();
        let port = opt.port.as_ref().unwrap();
        let path = opt.device_key.as_ref().unwrap();
        let body = opt.dumps();
        let content_type = if opt.encrypt.unwrap() {
            "application/x-www-form-urlencoded"
        } else {
            "application/json; charset=utf-8"
        };
        let buf = format!(
            "POST /{path} HTTP/1.1\r\n\
            Host: {host}\r\n\
            Content-Type: {content_type}\r\n\
            Content-Length: {}\r\n\
            Connection: close\r\n\
            \r\n\
            {body}",
            body.len(),
        );

        let mut root_store = RootCertStore::empty();
        root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
            OwnedTrustAnchor::from_subject_spki_name_constraints(
                ta.subject,
                ta.spki,
                ta.name_constraints,
            )
        }));
        let config = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        let server_name = host.try_into().expect("服务器解析错误!");
        let mut conn =
            rustls::ClientConnection::new(Arc::new(config), server_name).expect("连接服务器失败!");
        let mut sock = TcpStream::connect(format!("{host}:{port}")).expect("TCP 连接失败!");
        let mut tls = rustls::Stream::new(&mut conn, &mut sock);

        tls.write_all(&buf.as_bytes()).expect("发送请求错误!");

        let mut resp = String::with_capacity(256);
        // println!("开始接收");
        let _n = tls.read_to_string(&mut resp).expect("读取数据错误!");
        // println!("读到了 {} 字节", n);

        let body_start = resp.find("\r\n\r\n").unwrap_or(0) + 4;
        let resp_body = &resp[body_start..];

        // println!("{:?}", &buf);
        // println!("{:?}", &body);
        serde_json::from_str(resp_body).expect("解析错误, 返回结果无法解析")
    }
}
