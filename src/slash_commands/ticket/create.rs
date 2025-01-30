use std::collections::HashMap;

use serenity::all::{
    ButtonStyle, CommandInteraction, Context, CreateButton, CreateEmbed, CreateMessage, GuildId,
    ResolvedValue,
};
use sqlx::{Database, Pool};

use crate::{Result, TicketGuildManager};

use super::TicketCommand;

const DESCRIPTION_3: &str = "Congratulations on your win! Please create a ticket and send us your Bungie ID. Once you open a ticket, we will inform you if we need anything else.

Only <@381973220083105793> and the Moderation Team have access to the information in the ticket.";

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

        let embed = CreateEmbed::new().title(title).description(DESCRIPTION_3);

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
