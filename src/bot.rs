use crate::commands;
use crate::handler;
use crate::task::TaskLoader;
use crate::tools;
use reqwest::Proxy;
use std::error::Error;
use std::sync::Arc;
use teloxide::prelude::*;
pub struct MyBot {
    bot: Bot,
}

impl MyBot {
    pub fn new() -> Self {
        // 配置代理
        let client_builder = reqwest::Client::builder();
        let client = if let Ok(proxy_url) = std::env::var("TELEGRAM_PROXY") {
            log::info!("Using proxy: {}", proxy_url);
            client_builder
                .proxy(Proxy::https(&proxy_url).unwrap())
                .build()
                .unwrap()
        } else {
            client_builder.build().unwrap()
        };

        Self {
            bot: Bot::from_env_with_client(client),
        }
    }

    pub fn borrow_bot(&self) -> &Bot {
        &self.bot
    }

    pub async fn start(&self) {
        log::info!("Bot started");
        let _ = self
            .send_to("Bot started", tools::get_admin_id(), None)
            .await;

        let handler: Handler<
            '_,
            DependencyMap,
            Result<(), Box<dyn Error + Send + Sync>>,
            teloxide::dispatching::DpHandlerDescription,
        > = Update::filter_message()
            .branch(
                dptree::entry()
                    .filter_command::<commands::Command>()
                    .endpoint(handler::handle_command),
            )
            .branch(dptree::endpoint(handler::handle_general_message));

        Dispatcher::builder(self.bot.clone(), handler)
            .build()
            .dispatch()
            .await;
    }

    pub async fn send_to(
        &self,
        msg: &str,
        chat_id: String,
        topic: Option<i32>,
    ) -> Result<(), teloxide::RequestError> {
        if let Some(topic) = topic {
            self.borrow_bot()
                .send_message(chat_id, msg)
                .message_thread_id(topic)
                .await?;
        } else {
            self.borrow_bot().send_message(chat_id, msg).await?;
        }
        Ok(())
    }

    pub async fn worker(&self, task_loader: &mut TaskLoader) {
        use tokio::time::{interval, Duration};
        let mut ticker = interval(Duration::from_secs(10)); // 每10秒轮询一次

        loop {
            ticker.tick().await;

            let tasks = task_loader.load_tasks().await;
            for task in tasks {
                match task.send(&self).await {
                    Ok(_) => log::info!("message sent"),
                    Err(e) => log::error!("message send failed: {:?}", e),
                }
            }
        }
    }

    pub async fn bootstrap() {
        let bot = Arc::new(MyBot::new());

        let worker_bot = bot.clone();
        let start_bot = bot.clone();

        let worker_handle = tokio::spawn(async move {
            worker_bot.worker(&mut TaskLoader::new()).await;
        });
        let start_handle = tokio::spawn(async move {
            start_bot.start().await;
        });

        let _ = tokio::try_join!(worker_handle, start_handle);
    }
}
