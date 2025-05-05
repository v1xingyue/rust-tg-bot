use crate::bot::MyBot;
use std::collections::HashMap;
use std::error::Error;

pub struct SendTask {
    msg: String,
    chat_id: String,
    topic: Option<i32>,
}

mod task_load {

    use crate::task::SendTask;
    use crate::task::TaskLoader;
    use crate::tools;
    use std::error::Error;

    pub fn heartbeat() -> SendTask {
        return SendTask::new(
            format!(" Heartbeat : {} ", tools::now_string()),
            tools::get_admin_id(),
            None,
        );
    }

    pub async fn blend_tasks(
        loader: &mut TaskLoader,
    ) -> Result<Vec<SendTask>, Box<dyn Error + Send + Sync>> {
        let mut tasks = vec![];

        let time_tag = "blend_liquidation";
        let seconds_interval: u64 = 300;

        let last_time = loader.last_times.get(time_tag);
        if last_time.is_some() {
            let last_time = last_time.unwrap();
            if loader.unix_now() - last_time < seconds_interval {
                return Ok(vec![]);
            }
        }

        let blend_liquidataion_url = "https://liquidation-opportunities.flow.movefuns.xyz/edu.json";
        let blend_liquidataion_response = reqwest::get(blend_liquidataion_url).await?;

        let response = blend_liquidataion_response
            .json::<serde_json::Value>()
            .await?;

        let data = response["opportunities"].as_array().unwrap();

        let mut msg = String::new();

        for item in data {
            // "user_address": "0x44e8dCC0FbA2dE2b9370EdE1d838503CE4AE0A3A",
            // "collateral_token": "0x7277Cc818e3F3FfBb169c6Da9CC77Fc2d2a34895",
            // "debt_token": "0x7277Cc818e3F3FfBb169c6Da9CC77Fc2d2a34895",
            // "collateral_amount": "19470800000",
            // "debt_amount": "14430200000",
            // "max_liquidatable_debt": "7215100000",
            // "health_factor": 1.05246,
            // "estimated_profit_eth": 0.0,
            // "estimated_profit_usd": 721.853,

            //format this in one line

            let line_result = format!(
                "\n one possible liquidation \nuser_address: {} \n collateral_token: {}\n debt_token: {}\n collateral_amount: {}\n debt_amount: {}\n health_factor: {}\n estimated_profit_usd: {}\n",
                item["user_address"],
                item["collateral_token"],
                item["debt_token"],
                item["collateral_amount"],
                item["debt_amount"],
                item["health_factor"],
                item["estimated_profit_usd"],
            );

            msg.push_str(&line_result);
        }

        tasks.push(SendTask::new(msg, tools::get_admin_id(), None));

        loader.mark_time(time_tag);

        Ok(tasks)
    }

    pub async fn check_alive(
        _loader: &mut TaskLoader,
    ) -> Result<Vec<SendTask>, Box<dyn Error + Send + Sync>> {
        return Ok(vec![]);
    }
}

impl SendTask {
    pub fn new(msg: String, chat_id: String, topic: Option<i32>) -> Self {
        Self {
            msg,
            chat_id,
            topic,
        }
    }

    pub async fn send(&self, bot: &MyBot) -> Result<(), Box<dyn Error + Send + Sync>> {
        bot.send_to(&self.msg, self.chat_id.clone(), self.topic.clone())
            .await?;
        Ok(())
    }
}

pub struct TaskLoader {
    last_heartbeat: u64,
    last_times: HashMap<String, u64>,
}

impl TaskLoader {
    pub fn new() -> Self {
        Self {
            last_heartbeat: 0,
            last_times: HashMap::new(),
        }
    }

    pub fn unix_now(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    pub fn mark_time(&mut self, tag: &str) {
        self.last_times.insert(tag.to_string(), self.unix_now());
    }

    pub async fn load_tasks(&mut self) -> Vec<SendTask> {
        let mut tasks = vec![];
        let now = self.unix_now();

        if now - self.last_heartbeat > 30 {
            self.last_heartbeat = now;
            tasks.push(task_load::heartbeat());
        }

        let check_alive_task: Result<Vec<SendTask>, Box<dyn Error + Send + Sync>> =
            task_load::check_alive(self).await;
        if let Ok(check_alive_task) = check_alive_task {
            tasks.extend(check_alive_task);
        }

        let blend_tasks = task_load::blend_tasks(self).await;
        if let Ok(blend_tasks) = blend_tasks {
            tasks.extend(blend_tasks);
        }

        tasks
    }
}
