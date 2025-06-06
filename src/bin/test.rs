#![allow(unused_imports)]
#![allow(unused_variables)]
use QAQ::{ll_one_bot::interface::*, llm_api::interface::*, db::Database};
use QAQ::{API_SENDER, DATABASE_MANAGER, QQ_SENDER,initialize_database_manager};
use tokio::task::LocalSet;
use dotenv::dotenv;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    // let payload = SendBack::Private(SendBackPrivate{
    //     user_id: 2421468125,
    //     message: vec![QQMessage{
    //         r#type: "face".to_string(),
    //         data: MessageData::Face{id: "28".to_string()}
    //     },QQMessage{
    //         r#type: "text".to_string(),
    //         data: MessageData::Text{text: "lll".to_string()}
    //     }]
    // });
    // QQ_SENDER.send_qq_post(&payload).await?;
    initialize_database_manager().await;
    let db = DATABASE_MANAGER.get().unwrap();
    // db.generate_test_data().await?;
    db.reset_all_table().await?;
    //db.integrate_context(987654321,Some(123456789)).await?;
    Ok(())
}