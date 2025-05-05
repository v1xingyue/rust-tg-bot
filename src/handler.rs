use crate::commands::Command;
use crate::tools;
use std::error::Error;
use teloxide::prelude::*;

pub async fn handle_general_message(
    bot: Bot,
    msg: Message,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    tools::show_msg(&msg);

    let mut content = "".to_string();
    let thread_id = msg.thread_id;

    content.push_str("You talk from group: \n");
    content.push_str(&format!("group id: {}\n", msg.chat.id));

    if thread_id.is_some() {
        content.push_str(&format!("thread id: {}\n", thread_id.unwrap()));
    }

    content.push_str(&format!("chat id: {} \n", msg.chat.id));

    if thread_id.is_some() {
        bot.send_message(msg.chat.id, content)
            .message_thread_id(thread_id.unwrap())
            .await?;
    } else {
        bot.send_message(msg.chat.id, content).await?;
    }

    Ok(())
}

pub async fn handle_command(
    bot: Bot,
    msg: Message,
    cmd: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    tools::show_msg(&msg);
    cmd.execute(&bot, &msg).await?;
    Ok(())
}
