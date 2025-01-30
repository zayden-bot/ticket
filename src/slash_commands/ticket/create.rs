use std::collections::HashMap;

use serenity::all::{
    ButtonStyle, CommandInteraction, Context, CreateButton, CreateEmbed, CreateMessage, GuildId,
    ResolvedValue,
};
use sqlx::{Database, Pool};

use crate::{Result, TicketGuildManager};

use super::TicketCommand;

impl TicketCommand {
    pub(super) async fn create<Db: Database, GuildManager: TicketGuildManager<Db>>(
        ctx: &Context,
        interaction: &CommandInteraction,
        pool: &Pool<Db>,
        mut options: HashMap<&str, ResolvedValue<'_>>,
    ) -> Result<()> {
        let Some(ResolvedValue::String(title)) = options.remove("title") else {
            unreachable!("Title is required")
        };

        let Some(ResolvedValue::String(description)) = options.remove("description") else {
            unreachable!("Description is required")
        };

        let Some(ResolvedValue::String(label)) = options.remove("label") else {
            unreachable!("Label is required")
        };

        interaction.defer_ephemeral(ctx).await.unwrap();

        let embed = CreateEmbed::new().title(title).description(description);

        let button = CreateButton::new("ticket_create")
            .style(ButtonStyle::Primary)
            .label(label);

        interaction
            .channel_id
            .send_message(ctx, CreateMessage::new().embed(embed).button(button))
            .await
            .unwrap();

        Ok(())
    }
}
