use serenity::all::{
    AutoArchiveDuration, ChannelType, Context, CreateEmbed, CreateEmbedFooter,
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, CreateThread,
    Mentionable, ModalInteraction,
};
use sqlx::{Database, Pool};
use zayden_core::parse_modal_data;

use crate::{
    send_support_message, thread_name, ticket_manager::TicketManager, to_title_case, Result,
    TicketGuildManager,
};

pub struct TicketModal;

impl TicketModal {
    pub async fn run<
        Db: Database,
        GuildManager: TicketGuildManager<Db>,
        Manager: TicketManager<Db>,
    >(
        ctx: &Context,
        interaction: &ModalInteraction,
        pool: &Pool<Db>,
    ) -> Result<()> {
        let guild_id = interaction.guild_id.unwrap();

        let guild_row = GuildManager::get(pool, guild_id).await.unwrap().unwrap();

        let message = interaction.message.as_ref().unwrap();
        let ticket_row = Manager::get(pool, message.id).await.unwrap();
        let role_ids = ticket_row.role_ids();

        let mut data = parse_modal_data(&interaction.data.components);
        let content = data.remove("issue").unwrap();

        let thread_name = thread_name(
            guild_row.thread_id,
            interaction.member.as_ref().unwrap().display_name(),
            content,
        );

        let mut issue = CreateEmbed::new().title("Issue").description(content);

        if let Some(version) = data.remove("version") {
            issue = issue.footer(CreateEmbedFooter::new(version));
        }

        let mut messages: Vec<CreateMessage> = vec![CreateMessage::new().embed(issue)];

        data.drain()
            .filter(|(_, v)| !v.is_empty())
            .for_each(|(k, v)| {
                let title = to_title_case(k);
                let embed = CreateEmbed::new().title(title).description(v);
                messages.push(CreateMessage::new().embed(embed));
            });

        let additional = data.remove("additional").unwrap_or_default();
        if !additional.is_empty() {
            let additional = CreateEmbed::new()
                .title("Additional Information")
                .description(additional);

            messages[1] = CreateMessage::new().embed(additional);
        }

        let thread = interaction
            .channel_id
            .create_thread(
                ctx,
                CreateThread::new(&thread_name)
                    .kind(ChannelType::PrivateThread)
                    .auto_archive_duration(AutoArchiveDuration::OneWeek),
            )
            .await
            .unwrap();

        GuildManager::update_thread_id(pool, guild_id)
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

        Ok(())
    }
}
