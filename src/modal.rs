use serenity::all::{
    AutoArchiveDuration, ChannelType, Context, CreateEmbed, CreateEmbedFooter,
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, CreateThread,
    Mentionable, ModalInteraction,
};
use sqlx::{Database, Pool};
use zayden_core::parse_modal_data;

use crate::{send_support_message, thread_name, Error, Result, TicketGuildManager};

pub struct SupportModal;

impl SupportModal {
    pub async fn run<Db: Database, GuildManager: TicketGuildManager<Db>>(
        ctx: &Context,
        interaction: &ModalInteraction,
        pool: &Pool<Db>,
    ) -> Result<()> {
        let guild_id = interaction.guild_id.ok_or(Error::MissingGuildId)?;

        let row = GuildManager::get(pool, guild_id).await.unwrap().unwrap();

        let channel_id = row.channel_id().unwrap();
        let role_ids = row.role_ids();

        let mut data = parse_modal_data(&interaction.data.components);
        let content = data.remove("issue").unwrap();

        let thread_name = thread_name(
            row.thread_id,
            interaction.member.as_ref().unwrap().display_name(),
            content,
        );

        let version = data.remove("version").unwrap();

        let issue = CreateEmbed::new()
            .title("Issue")
            .description(content)
            .footer(CreateEmbedFooter::new(format!("Version: {}", version)));

        let mut messages: [CreateMessage; 2] =
            [CreateMessage::new().embed(issue), CreateMessage::new()];

        let additional = data.remove("additional").unwrap();
        if !additional.is_empty() {
            let additional = CreateEmbed::new()
                .title("Additional Information")
                .description(additional);

            messages[1] = CreateMessage::new().embed(additional);
        }

        let thread = channel_id
            .create_thread(
                ctx,
                CreateThread::new(&thread_name)
                    .kind(ChannelType::PrivateThread)
                    .auto_archive_duration(AutoArchiveDuration::OneWeek),
            )
            .await
            .unwrap();

        interaction
            .create_response(
                ctx,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content(format!("Support thread created: {}", thread.mention()))
                        .ephemeral(true),
                ),
            )
            .await
            .unwrap();

        send_support_message(ctx, thread.id, &role_ids, &interaction.user, messages)
            .await
            .unwrap();

        // update_support_thread_id(&pool, guild_id.get(), thread_id)
        //     .await
        //     .unwrap();

        Ok(())
    }
}
