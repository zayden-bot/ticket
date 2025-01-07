pub mod components;
pub mod error;
pub mod message_command;
pub mod modal;
pub mod slash_commands;
pub mod support_guild_manager;

pub use error::Error;
use error::Result;
use serenity::all::{
    ButtonStyle, ChannelId, Context, CreateButton, CreateMessage, Mentionable, RoleId, User,
};
pub use support_guild_manager::TicketGuildManager;

pub fn thread_name(thread_id: i32, author_name: &str, content: &str) -> String {
    format!("{} - {} - {}", thread_id, author_name, content)
        .chars()
        .take(100)
        .collect()
}

pub async fn send_support_message(
    ctx: &Context,
    thread_id: ChannelId,
    role_ids: &[RoleId],
    author: &User,
    messages: [CreateMessage; 2],
) -> Result<()> {
    let mentions: String = role_ids
        .iter()
        .map(|role| role.mention().to_string())
        .chain([author.mention().to_string()])
        .collect();

    let [first, last] = messages;

    thread_id
        .send_message(ctx, first.content(mentions))
        .await
        .unwrap();

    let button = CreateButton::new("support_close")
        .label("Close")
        .style(ButtonStyle::Primary);

    thread_id
        .send_message(ctx, last.button(button))
        .await
        .unwrap();

    Ok(())
}
