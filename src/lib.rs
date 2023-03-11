pub mod types {
    use serde::Deserialize;
    use std::{error, fmt};
    use structopt::StructOpt;
    use url_builder::URLBuilder;

    #[derive(Debug, Clone)]
    pub struct LackError;

    impl fmt::Display for LackError {
        fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
            todo!()
        }
    }
    impl error::Error for LackError {
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            todo!()
        }
    }

    #[derive(Debug, StructOpt)]
    #[structopt(
        name = "bark.rs",
        about = "Bark cli by Rust.",
        version = "1.0",
        author = "kc9vu"
    )]
    pub struct Opt {
        /// 配置文件, 缺少必需设置时从中查找. 不指定时从程序目录和系统目录中查找
        #[structopt(short, long)]
        pub file: Option<String>,

        /// 服务器地址
        #[structopt(short, long)]
        pub host: Option<String>,

        /// 服务器端口
        #[structopt(short, long)]
        pub port: Option<u16>,

        /// 服务器注册 key (必需)
        #[structopt(short, long)]
        pub key: Option<String>,

        /// 推送加密, 暂未实现
        #[structopt(short = "P", long)]
        pub cipher: Option<String>,

        /// 标题
        #[structopt(short, long)]
        pub title: Option<String>,

        /// 自动复制
        #[structopt(short, long)]
        pub copy: bool,

        /// 复制内容
        #[structopt(short = "C", long)]
        pub content: Option<String>,

        /// 链接
        #[structopt(short = "U", long)]
        pub url: Option<String>,

        /// 是否保存: 不指定时遵守客户端设置, 1 个标志保存, 多个标志不保存
        #[structopt(short = "A", parse(from_occurrences))]
        pub archive: i8,

        /// 通知等级: 不指定为默认通知, 1 个标志为时效性通知, 多个标志为仅添加到通知列表
        #[structopt(short = "L", parse(from_occurrences))]
        pub level: u8,

        /// 分组
        #[structopt(short, long)]
        pub group: Option<String>,

        /// 角标, 可为任意值
        #[structopt(short = "B", long)]
        pub badge: Option<u16>,

        /// 图标, 支持 iOS15 及以上
        #[structopt(short = "I", long)]
        pub icon: Option<String>,

        /// 铃声
        #[structopt(short = "S", long)]
        pub sound: Option<String>,

        /// 消息
        #[structopt(name = "message")]
        pub message: String,
    }

    impl Opt {
        pub fn is_invalid(&self) -> bool {
            self.host.is_none() || self.port.is_none() || self.key.is_none()
        }

        pub fn patch(&mut self, cfg: Config) -> Result<(), LackError> {
            if self.host.is_none() {
                if let Some(host) = cfg.host {
                    self.host = Some(host);
                } else {
                    self.host = Some("https://api.day.app".to_string());
                }
            }

            if self.port.is_none() {
                if let Some(port) = cfg.port {
                    self.port = Some(port);
                } else {
                    self.port = Some(443);
                }
            }

            if self.key.is_none() {
                if let Some(key) = cfg.key {
                    self.key = Some(key);
                } else {
                    return Err(LackError);
                }
            }

            if self.is_invalid() {
                Err(LackError)
            } else {
                Ok(())
            }
        }

        pub fn build_url(&self) -> Result<String, LackError> {
            if self.is_invalid() {
                return Err(LackError);
            }

            let mut ub = URLBuilder::new();
            ub.set_protocol("https")
                .set_host(self.host.as_ref().unwrap())
                .set_port(self.port.unwrap())
                .add_route(self.key.as_ref().unwrap());
            if self.title.is_some() {
                ub.add_route(self.title.as_ref().unwrap());
            }
            ub.add_route(&self.message);

            if self.cipher.is_some() {
                println!("暂未实现, 敬请期待!");
                todo!();
            } else {
                if self.copy {
                    ub.add_param("autoCopy", "1");
                }
                if let Some(content) = self.content.as_ref() {
                    ub.add_param("copy", content);
                }
                if let Some(url) = self.url.as_ref() {
                    ub.add_param("url", url);
                }
                if self.archive == 1 {
                    ub.add_param("isArchive", "1");
                } else if self.archive != 0 {
                    ub.add_param("isArchive", "0");
                }
                let level = match self.level {
                    0 => "active",
                    1 => "timeSensitive",
                    _ => "passive",
                };
                ub.add_param("level", level);
                if let Some(group) = self.group.as_ref() {
                    ub.add_param("group", group);
                }
                if let Some(badge) = self.badge.as_ref() {
                    ub.add_param("badge", &badge.to_string());
                }
                if let Some(icon) = self.icon.as_ref() {
                    ub.add_param("icon", icon);
                }
            }

            Ok(ub.build())
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct Config {
        pub host: Option<String>,
        pub port: Option<u16>,
        pub key: Option<String>,
    }
}

pub mod cfg {
    use super::types::{Config, Opt};
    use std::{env, fs, path::Path};
    use structopt::StructOpt;

    pub fn options() -> Option<Opt> {
        let mut opt = Opt::from_args();
        if opt.is_invalid() {
            if let Some(config) = parse_config(&opt.file) {
                if opt.patch(config).is_err() {
                    return None;
                }
            }
        }
        Some(opt)
    }

    pub fn parse_config(file: &Option<String>) -> Option<Config> {
        if let Some(file) = file {
            return read_config(file);
        } else if let Some(file) = config_dir() {
            return read_config(&file);
        }
        None
    }

    pub fn read_config(path: &str) -> Option<Config> {
        if let Ok(string) = fs::read_to_string(path) {
            if let Ok(json) = serde_json::from_str(&string) {
                return Some(json);
            }
        }
        None
    }

    pub fn config_dir() -> Option<String> {
        match env::consts::OS {
            "windows" => windows_config_dir(),
            "linux" => None,
            "macos" => None,
            _ => None,
        }
    }

    fn windows_config_dir() -> Option<String> {
        let path = env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("bark-cli.json");
        if path.exists() {
            return Some(String::from(path.to_str().unwrap()));
        }

        if let Ok(path) = env::var("APPDATA") {
            let path = Path::new(&path).join("bark-cli").join("bark-cli.json");
            if path.exists() {
                return Some(String::from(path.to_str().unwrap()));
            }
        }

        if let Ok(path) = env::var("HOME") {
            let path = Path::new(&path).join("bark-cli.json");
            if path.exists() {
                return Some(String::from(path.to_str().unwrap()));
            }
        }

        if let Ok(path) = env::var("USERPROFILE") {
            let path = Path::new(&path).join("bark-cli.json");
            if path.exists() {
                return Some(String::from(path.to_str().unwrap()));
            }
        }
        None
    }
}

pub fn push() {}
