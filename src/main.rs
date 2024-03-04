mod nbnhhsh;

use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use teloxide::{Bot, dptree};
use teloxide::dispatching::{Dispatcher, UpdateFilterExt};
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{InlineQuery, Message, Requester, Update};
use teloxide::types::{InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText, Me};
use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "能不能好好说话 Bot 使用说明
上游地址： https://github.com/itorr/nbnhhsh
机器人地址： https://github.com/imlonghao/nbnhhsh_bot")]
enum Command {
    Start,
    #[command(description = "显示帮助信息")]
    Help,
}


#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot...");

    let bot = Bot::from_env();

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_inline_query().endpoint(inline_query_handler));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn nbnhhsh_process(q: String, bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    let guess_result = nbnhhsh::guess(q).await.unwrap();
    let mut reply: String = "".to_string();
    guess_result.iter().for_each(|x| {
        let mut descrition = "找不到相关信息".to_string();
        if x.trans.len() > 0 {
            descrition = x.trans.join(", ");
        } else if x.inputting.len() > 0 {
            descrition = format!("(?) {}", x.inputting.join(", "));
        }
        let m = format!("{}: {}\n", x.name, descrition);
        reply += &*m;
    });
    if reply == "" {
        reply = "无数据".to_string();
    }
    bot.send_message(msg.chat.id, reply)
        .reply_to_message_id(msg.id)
        .await?;
    Ok(())
}

async fn message_handler(
    bot: Bot,
    msg: Message,
    me: Me,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(text) = msg.text() {
        println!("[msg][{}] {}", msg.from().unwrap().id, text);

        match BotCommands::parse(text, me.username()) {
            Ok(Command::Start) | Ok(Command::Help) => {
                bot.send_message(
                    msg.chat.id, Command::descriptions().to_string())
                    .reply_to_message_id(msg.id)
                    .disable_web_page_preview(true)
                    .await?;
            }
            Err(_) => {
                nbnhhsh_process(text.to_string(), bot, msg).await?;
            }
        }
    }

    Ok(())
}

async fn inline_query_handler(
    bot: Bot,
    q: InlineQuery,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("[inline][{}] {}", q.from.id, q.query);

    let guess_result = nbnhhsh::guess(q.query).await.unwrap();
    let mut resp_payload: Vec<InlineQueryResult> = vec![];

    guess_result.iter().for_each(|x| {
        let mut descrition = "找不到相关信息".to_string();
        if x.trans.len() > 0 {
            descrition = x.trans.join(", ");
        } else if x.inputting.len() > 0 {
            descrition = format!("(?) {}", x.inputting.join(", "));
        }
        let m = format!("{}: {}", x.name, descrition);
        let article = InlineQueryResultArticle::new(
            format!("{}_{}", x.name, SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()),
            x.name.clone(),
            InputMessageContent::Text(InputMessageContentText::new(m)),
        ).description(descrition);
        resp_payload.push(InlineQueryResult::Article(article))
    });

    bot.answer_inline_query(q.id, resp_payload).await?;

    Ok(())
}