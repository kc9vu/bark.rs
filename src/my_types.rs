use std::error::Error;

use base64::prelude::{Engine as _, BASE64_STANDARD};
use openssl::symm::{encrypt, Cipher};
use serde::Deserialize;
use structopt::{clap::ArgGroup, StructOpt};

use super::{app, my_errors::LackError};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "bark.rs",
    about = "Bark cli by Rust.",
    version = "2.2.1",
    author = "kc9vu",
    group = ArgGroup::with_name("level_group")
)]
pub struct Opt {
    /// 配置文件, 缺少必需设置时从中查找. 不指定时从程序目录中查找
    #[structopt(short, long)]
    pub file: Option<String>,

    /// 服务器地址
    #[structopt(short, long)]
    pub host: Option<String>,

    /// 服务器端口
    #[structopt(short, long)]
    pub port: Option<u16>,

    /// 设备标识 (必需)
    #[structopt(short, long)]
    pub device_key: Option<String>,

    /// 消息
    #[structopt(short, long)]
    pub body: String,

    /// 标题
    #[structopt(short, long)]
    pub title: Option<String>,

    /// 自动复制
    #[structopt(short, long)]
    pub auto_copy: bool,

    /// 复制内容
    #[structopt(short, long)]
    pub copy: Option<String>,

    /// 链接
    #[structopt(short, long)]
    pub url: Option<String>,

    /// 是否保存通知
    #[structopt(short = "A", long)]
    pub is_archive: Option<bool>,

    /// 通知等级: 默认通知 (active), 时效性通知 (timeSensitive), 仅添加到通知列表 (passive)
    #[structopt(short, long, group = "level_group")]
    pub level: Option<String>,

    /// 默认通知
    #[structopt(long, group = "level_group")]
    pub active: bool,

    /// 时效性通知
    #[structopt(long, group = "level_group")]
    pub time_sensitive: bool,

    /// 仅添加到通知列表
    #[structopt(long, group = "level_group")]
    pub passive: bool,

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

    /// 推送加密, 如果 key 和 iv 存在且没有指定 encrpt 时会自动设为 true
    #[structopt(short, long)]
    pub encrypt: Option<bool>,

    /// 加密用 key, 只支持 AES256 CBC
    #[structopt(long)]
    pub key: Option<String>,

    /// 加密用 iv
    #[structopt(long)]
    pub iv: Option<String>,
}

impl Opt {
    pub fn invalid_message(&self) -> Option<&str> {
        if self.host.is_none() || self.port.is_none() || self.device_key.is_none() {
            return Some("缺少必需的服务器配置或设备标识!");
        }
        if let Some(true) = self.encrypt {
            if self.key.is_none() || self.iv.is_none() {
                return Some("加密密钥必须同时提供, 包括配置文件中的!");
            }
        }
        None
    }

    pub fn check(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(path) = &self.file {
            self.patch(app::read_file_config(path)?)?;
        } else if let Some(path) = app::execute_exe_config() {
            self.patch(app::read_file_config(&path)?)?;
        }
        if let Some(message) = self.invalid_message() {
            Err(Box::new(LackError::from(message)))
        } else {
            Ok(())
        }
    }

    pub fn patch(&mut self, conf: Conf) -> Result<(), Box<dyn Error>> {
        if self.host.is_none() {
            if let Some(host) = conf.host {
                self.host = Some(host);
            } else {
                self.host = Some("api.day.app".to_string());
            }
        }
        if self.port.is_none() {
            if let Some(port) = conf.port {
                self.port = Some(port);
            } else {
                self.port = Some(443);
            }
        }
        if self.device_key.is_none() {
            if let Some(device_key) = conf.device_key {
                self.device_key = Some(device_key);
            } else {
                return Err(Box::new(LackError::from("缺少必要的设备标识!")));
            }
        }
        if self.title.is_none() {
            if let Some(title) = conf.title {
                self.title = Some(title);
            }
        }
        // if self.auto_copy.is_none() {
        //     if let Some(auto_copy) = conf.auto_copy {
        //         self.auto_copy = Some(auto_copy);
        //     }
        // }
        if self.is_archive.is_none() {
            if let Some(is_archive) = conf.is_archive {
                self.is_archive = Some(is_archive);
            }
        }
        if self.level.is_none() && !(self.active || self.time_sensitive || self.passive) {
            if let Some(level) = conf.level {
                self.level = Some(level);
            } else {
                self.level = Some("active".to_string());
            }
        }
        if self.group.is_none() {
            if let Some(group) = conf.group {
                self.group = Some(group);
            }
        }
        if self.icon.is_none() {
            if let Some(icon) = conf.icon {
                self.icon = Some(icon);
            }
        }
        if self.sound.is_none() {
            if let Some(sound) = conf.sound {
                self.sound = Some(sound);
            }
        }

        if self.encrypt.is_none() {
            if let Some(encrypt) = conf.encrypt {
                self.encrypt = Some(encrypt);
            }
        }
        if self.key.is_none() && self.iv.is_none() {
            if let Some(key) = conf.key {
                self.key = Some(key);
            }
            if let Some(iv) = conf.iv {
                self.iv = Some(iv);
            }
        }

        if self.key.is_some() && self.iv.is_some() && self.encrypt.is_none() {
            self.encrypt = Some(true);
        }

        // 加密提示
        if let Some(true) = self.encrypt {
            println!("由于未知原因(与新的请求方法无关), 加密暂时可能不可用");
        }

        if let Some(message) = self.invalid_message() {
            Err(Box::new(LackError::from(message)))
        } else {
            Ok(())
        }
    }

    pub fn dumps(&self) -> String {
        let mut json = String::new();
        let mut items = Vec::new();

        json.push('{');

        items.push(json_pair("body", &quote_str(&self.body)));
        if let Some(title) = &self.title {
            items.push(json_pair("title", &quote_str(title)));
        }
        if self.auto_copy {
            items.push(json_pair("autoCopy", "true"));
        }
        if let Some(copy) = &self.copy {
            items.push(json_pair("copy", &quote_str(copy)));
        }
        if let Some(url) = &self.url {
            items.push(json_pair("url", &quote_str(url)));
        }
        if let Some(archive) = &self.is_archive {
            items.push(json_pair("isArchive", if *archive { "1" } else { "0" }));
        }
        if let Some(level) = &self.level {
            items.push(json_pair("level", &quote_str(level)));
        } else {
            items.push(json_pair(
                "level",
                if self.time_sensitive {
                    "\"timeSensitive\""
                } else if self.passive {
                    "\"passive\""
                } else {
                    "\"active\""
                },
            ));
        }
        // items.push(json_pair("level", self.level.as_ref().unwrap()));
        if let Some(group) = &self.group {
            items.push(json_pair("group", &quote_str(group)));
        }
        if let Some(badge) = &self.badge {
            items.push(json_pair("badge", &badge.to_string()));
        }
        if let Some(icon) = &self.icon {
            items.push(json_pair("icon", &quote_str(icon)));
        }
        if let Some(sound) = &self.sound {
            items.push(json_pair("sound", &quote_str(sound)));
        }

        json.push_str(&items.join(","));
        json.push('}');

        if let Some(true) = self.encrypt {
            format!("ciphertext={}", self.enc(&json).replace('=', "%3D"))
        } else {
            json
        }
    }

    fn enc(&self, input: &str) -> String {
        let input = input.as_bytes();

        let mut key = [0; 32];
        let mut iv = [0; 16];

        BASE64_STANDARD
            .decode_slice_unchecked(self.key.as_ref().unwrap(), &mut key)
            .expect("加密 key 不可用!");
        BASE64_STANDARD
            .decode_slice_unchecked(self.iv.as_ref().unwrap(), &mut iv)
            .expect("加密 iv 不可用!");

        let cipher = Cipher::aes_256_cbc();
        BASE64_STANDARD.encode(encrypt(cipher, &key, Some(&iv), input).unwrap())
    }
}

#[derive(Deserialize)]
pub struct Conf {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub device_key: Option<String>,
    pub title: Option<String>,
    pub is_archive: Option<bool>,
    pub level: Option<String>,
    pub group: Option<String>,
    pub icon: Option<String>,
    pub sound: Option<String>,
    pub encrypt: Option<bool>,
    pub key: Option<String>,
    pub iv: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Resp {
    pub code: u16,
    pub message: String,
    pub timestamp: u32,
}

fn json_pair(key: &str, value: &str) -> String {
    format!("\"{}\":{}", key, value)
}
fn quote_str(s: &str) -> String {
    format!("\"{}\"", s)
}
