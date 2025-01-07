use futures::{stream, StreamExt};
use serenity::all::{
    AutoArchiveDuration, ChannelType, Context, CreateAttachment, CreateEmbed, CreateMessage,
    CreateThread, DiscordJsonError, ErrorResponse, HttpError, Message,
};
use sqlx::{Database, Pool};

use crate::{send_support_message, thread_name, Error, Result, TicketGuildManager};

pub struct SupportMessageCommand;

impl SupportMessageCommand {
    pub async fn run<Db: Database, GuildManager: TicketGuildManager<Db>>(
        ctx: &Context,
        message: &Message,
        pool: &Pool<Db>,
    ) -> Result<()> {
        let guild_id = message.guild_id.ok_or(Error::MissingGuildId)?;

        let row = GuildManager::get(pool, guild_id).await.unwrap().unwrap();

        let channel_id = row.channel_id().unwrap();
        if channel_id != message.channel_id {
            return Ok(());
        }

        let role_ids = row.role_ids();

        let thread_name = thread_name(
            row.thread_id,
            message.author.display_name(),
            &message.content,
        );

        let thread = message
            .channel_id
            .create_thread(
                &ctx,
                CreateThread::new(thread_name)
                    .kind(ChannelType::PrivateThread)
                    .auto_archive_duration(AutoArchiveDuration::OneWeek),
            )
            .await?;

        let issue = CreateEmbed::new()
            .title("Issue")
            .description(&message.content);

        let attachments = stream::iter(message.attachments.iter())
            .then(|attachment| async move {
                CreateAttachment::bytes(
                    attachment.download().await.unwrap(),
                    attachment.filename.clone(),
                )
            })
            .collect::<Vec<_>>()
            .await;

        let messages: [CreateMessage; 2] = [
            CreateMessage::new().embed(issue).files(attachments),
            CreateMessage::new(),
        ];

        send_support_message(ctx, thread.id, &role_ids, &message.author, messages)
            .await
            .unwrap();

        match message.delete(&ctx).await {
            // 10008: Unknown Message
            Err(serenity::Error::Http(HttpError::UnsuccessfulRequest(ErrorResponse {
                error: DiscordJsonError { code: 10008, .. },
                ..
            }))) => {}
            result => {
                result.unwrap();
            }
        }

        Ok(())
    }
}
