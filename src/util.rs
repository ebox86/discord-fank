use serenity::{
    builder::CreateMessage,
    model::{
      channel::Message,
      id::ChannelId,
    }, http::Http,
  };
  
  pub async fn send_message(http: &Http, channel_id: u64, msg: CreateMessage<'_>) -> serenity::Result<Message> {
    let channel_id = ChannelId(channel_id);
    channel_id.send_message(http, |m| { *m = msg; m }).await
  }