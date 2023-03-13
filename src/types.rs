
use std::error::Error;
use serde::Deserialize;
use structopt::StructOpt;
use errors::LackError;

pub mod errors;

#[derive(Debug, StructOpt)]
#[structopt(
name = "bark.rs",
about = "Bark cli by Rust.",
version = "1.1",
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

    pub fn patch(&mut self, cfg: Config) -> Result<(), Box<dyn Error>> {
        if self.key.is_none() {
            if let Some(key) = cfg.key {
                self.key = Some(key);
            } else {
                return Err(Box::new(LackError::from("缺少必要的 key！")));
            }
        }

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

        if self.is_invalid() { Err(Box::new(LackError::from("缺少必要的配置！"))) } else { Ok(()) }
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub key: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Resp {
    pub code: u16,
    pub message: String,
    pub timestamp: u32,
}