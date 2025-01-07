use futures::{StreamExt, TryStreamExt};
use serenity::all::{
    ComponentInteraction, ComponentInteractionDataKind, Context, CreateEmbed,
    CreateInteractionResponse, CreateInteractionResponseMessage,
};
use sqlx::{Database, Pool};

use crate::{Error, Result, TicketGuildManager};

pub async fn support_faq<Db: Database, GuildManager: TicketGuildManager<Db>>(
    ctx: &Context,
    interaction: &ComponentInteraction,
    pool: &Pool<Db>,
) -> Result<()> {
    let guild_id = interaction.guild_id.ok_or(Error::MissingGuildId)?;

    let index = match &interaction.data.kind {
        ComponentInteractionDataKind::StringSelect { values } => {
            values[0].parse::<usize>().unwrap()
        }
        _ => unreachable!("Invalid interaction data kind"),
    };

    let faq_channel_id = GuildManager::get(pool, guild_id)
        .await
        .unwrap()
        .unwrap()
        .faq_channel_id();

    let message = faq_channel_id
        .messages_iter(ctx)
        .skip(index)
        .boxed()
        .try_next()
        .await
        .unwrap()
        .unwrap();

    let mut parts: Vec<&str> = message.content.split("**").collect();
    let description = parts.pop().unwrap().trim();
    let title = parts.join("");

    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().embed(
                    CreateEmbed::new()
                        .title(title.trim())
                        .description(description),
                ),
            ),
        )
        .await?;

    Ok(())
}
