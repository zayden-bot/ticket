pub mod components;
pub mod ready;
pub use components::TicketComponent;

pub mod error;
pub use error::Error;
use error::Result;

pub mod message_command;
pub use message_command::SupportMessageCommand;

pub mod modal;
pub use modal::SupportModal;

pub mod slash_commands;
pub use slash_commands::SupportCommand;
pub use slash_commands::TicketCommand;

pub mod support_guild_manager;
pub use support_guild_manager::TicketGuildManager;

use serenity::all::{
    ButtonStyle, ChannelId, Context, CreateButton, CreateMessage, Mentionable, RoleId, User,
};

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
