use std::collections::HashMap;

use serenity::all::{
    ButtonStyle, CommandInteraction, Context, CreateButton, CreateEmbed, CreateMessage, GuildId,
    ResolvedValue,
};
use sqlx::{Database, Pool};

use crate::{Result, TicketGuildManager};

use super::TicketCommand;

const DESCRIPTION_1: &str = "This is a safe way to report an issue with the server or voice your concerns with another community member.

Only <@381973220083105793> and the <@&1275143477654454394> Team can access the information in this Support Ticket.";

const DESCRIPTION_2: &str = "This is a safe way to report any issues in the channel or voice your concerns regarding another viewer.

Only <@381973220083105793> and the <@&1275149982701191260> Team can access the information in this Support Ticket.";

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

        let embed = CreateEmbed::new().title(title).description(DESCRIPTION_1);

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
