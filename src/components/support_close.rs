use serenity::all::{ComponentInteraction, Context, CreateInteractionResponse, EditThread};

use crate::Result;

pub async fn support_close(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
    let channel = interaction.channel.as_ref().unwrap();

    let new_channel_name: String = format!("{} - {}", "[Closed]", channel.name.as_ref().unwrap())
        .chars()
        .take(100)
        .collect();

    interaction
        .channel_id
        .edit_thread(ctx, EditThread::new().name(new_channel_name).archived(true))
        .await?;

    interaction
        .create_response(ctx, CreateInteractionResponse::Acknowledge)
        .await?;

    Ok(())
}
