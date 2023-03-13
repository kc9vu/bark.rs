use std::{env, error::Error, fs, path::Path};
use structopt::StructOpt;
use url_builder::URLBuilder;
use super::types::{
    Config,
    Opt,
    Resp,
    errors::*,
};

pub fn options() -> Result<Opt, Box<dyn Error>> {
    let mut opt = Opt::from_args();
    if opt.is_invalid() {
    //     if let Some(config) = parse_config(&opt.file) {
    //         opt.patch(config)?;
    //     }
        opt.patch(parse_config(&opt.file)?)?;
    }
    Ok(opt)
}

pub fn parse_config(file: &Option<String>) -> Result<Config, Box<dyn Error>> {
    if let Some(file) = file {
        read_config(file)
    } else if let Some(path) = config_dir() {
        read_config(&path)
    } else {
        Err(Box::new(LackError::from("缺少必要的配置！")))
    }
    // else {
    //     read_config(&config_dir().unwrap())
    // }
}

pub fn read_config(path: &str) -> Result<Config, Box<dyn Error>> {
    Ok(serde_json::from_str::<Config>(&fs::read_to_string(path)?)?)
}

pub fn config_dir() -> Option<String> {
    match env::consts::OS {
        "windows" => match windows_config_dir() {
            Some(path) => Some(path),
            None => None,
        },
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

pub fn build_url(opt: &Opt) -> Result<String, Box<dyn Error>> {
    if opt.is_invalid() {
        return Err(Box::new(LackError::from("缺少必要的配置！")));
    }

    let mut ub = URLBuilder::new();
    ub.set_protocol("https")
        .set_host(opt.host.as_ref().unwrap())
        .set_port(opt.port.unwrap())
        .add_route(opt.key.as_ref().unwrap());
    if opt.title.is_some() {
        ub.add_route(opt.title.as_ref().unwrap());
    }
    ub.add_route(&opt.message);

    if opt.cipher.is_some() {
        println!("暂未实现, 敬请期待!");
        todo!();
    } else {
        if opt.copy {
            ub.add_param("autoCopy", "1");
        }
        if let Some(content) = opt.content.as_ref() {
            ub.add_param("copy", content);
        }
        if let Some(url) = opt.url.as_ref() {
            ub.add_param("url", url);
        }
        if opt.archive == 1 {
            ub.add_param("isArchive", "1");
        } else if opt.archive != 0 {
            ub.add_param("isArchive", "0");
        }
        let level = match opt.level {
            0 => "active",
            1 => "timeSensitive",
            _ => "passive",
        };
        ub.add_param("level", level);
        if let Some(group) = opt.group.as_ref() {
            ub.add_param("group", group);
        }
        if let Some(badge) = opt.badge.as_ref() {
            ub.add_param("badge", &badge.to_string());
        }
        if let Some(icon) = opt.icon.as_ref() {
            ub.add_param("icon", icon);
        }
    }

    Ok(ub.build())
}

pub fn push(url: &str) -> Result<Resp, Box<dyn Error>> {
    Ok(reqwest::blocking::get(url)?.json::<Resp>()?)
}