use chrono::Local;

use teloxide::prelude::*;

pub fn show_msg(msg: &Message) {
    log::info!(
        "received message: {} chat_id: {} title: {}",
        msg.text().unwrap(),
        msg.chat.id,
        msg.chat
            .title()
            .unwrap_or(msg.chat.username().unwrap_or("unknown")),
    );
}

pub fn log_init() {
    // 设置 info 为默认日志级别
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();
}

pub fn get_admin_id() -> String {
    std::env::var("ADMIN_ID").unwrap()
}

pub fn now_string() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}
