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
    mut messages: Vec<CreateMessage>,
) -> Result<()> {
    let mentions: String = role_ids
        .iter()
        .map(|role| role.mention().to_string())
        .chain([author.mention().to_string()])
        .collect();

    let button = CreateButton::new("support_close")
        .label("Close")
        .style(ButtonStyle::Primary);

    if messages.len() == 1 {
        thread_id
            .send_message(
                ctx,
                messages.pop().unwrap().content(mentions).button(button),
            )
            .await
            .unwrap();

        return Ok(());
    }

    let last_idx = messages.len() - 1;

    for (i, message) in messages.into_iter().enumerate() {
        if i == 0 {
            thread_id
                .send_message(ctx, message.content(mentions.clone()))
                .await
                .unwrap();

            continue;
        }

        if i == last_idx {
            thread_id
                .send_message(ctx, message.button(button.clone()))
                .await
                .unwrap();

            continue;
        }

        thread_id.send_message(ctx, message).await.unwrap();
    }

    Ok(())
}

fn to_title_case(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    first.to_uppercase().collect::<String>()
                        + chars.as_str().to_lowercase().as_str()
                }
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}
