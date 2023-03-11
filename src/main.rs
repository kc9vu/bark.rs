use bark_cli::cfg::options;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let opt = options().expect("配置错误, 请检查后重试!");
    let url = opt.build_url().expect("无法创建链接, 请检查后重试!");
    let status = reqwest::get(&url)
        .await
        .expect("无法连接服务器, 请检查网络连接或服务器配置!")
        .status();
    println!("{}", status);
}
