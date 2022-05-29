use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: Option<ServerConfig>,
    pub bot: Option<BotConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    // 服务器端口
    pub port: Option<u16>,
}

#[derive(Debug, Deserialize)]
pub struct BotConfig {
    // Telegram 机器人 token
    pub token: Option<String>,
    // 管理员
    pub admins: Option<Vec<u64>>,
    // 发布频道
    pub report_channel_id: Option<i64>,
    // 待处理频道
    pub report_chat_id: Option<i64>,
}

impl Config {
    pub fn parse(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}

// 数据库地址
pub const DATABASE_URL: &str = "sqlite:black.db";
