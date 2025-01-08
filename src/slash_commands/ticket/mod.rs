mod close;
mod fixed;
mod open;

use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    Permissions, ResolvedOption, ResolvedValue,
};
use sqlx::{Database, Pool};
use zayden_core::parse_options;

use crate::{Error, Result, TicketGuildManager};

pub struct TicketCommand;

impl TicketCommand {
    pub async fn run<Db: Database, GuildManager: TicketGuildManager<Db>>(
        ctx: &Context,
        interaction: &CommandInteraction,
        pool: &Pool<Db>,
        options: Vec<ResolvedOption<'_>>,
    ) -> Result<()> {
        let guild_id = interaction.guild_id.ok_or(Error::MissingGuildId)?;

        let command = &options[0];

        let options = match &command.value {
            ResolvedValue::SubCommand(options) => options,
            ResolvedValue::SubCommandGroup(options) => options,
            _ => unreachable!("Subcommand is required"),
        };
        let options = parse_options(options);

        match command.name {
            "close" => {
                Self::close::<Db, GuildManager>(ctx, interaction, pool, options, guild_id).await?
            }
            "fixed" => {
                Self::fixed::<Db, GuildManager>(ctx, interaction, pool, options, guild_id).await?
            }
            "open" => Self::open::<Db, GuildManager>(ctx, interaction, pool, guild_id).await?,
            _ => unreachable!("Subcommand is required"),
        }

        Ok(())
    }

    pub fn register() -> CreateCommand {
        let close =
            CreateCommandOption::new(CommandOptionType::SubCommand, "close", "Close the ticket")
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "message",
                        "Message to send before closing the ticket",
                    )
                    .required(false),
                );

        let fixed = CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "fixed",
            "Mark the ticket as fixed",
        )
        .add_sub_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "version",
                "Version of the game that fixed the issue",
            )
            .required(false),
        );

        let open =
            CreateCommandOption::new(CommandOptionType::SubCommand, "open", "Open the ticket");

        CreateCommand::new("ticket")
            .description("Ticket management commands")
            .default_member_permissions(Permissions::MANAGE_MESSAGES)
            .add_option(close)
            .add_option(fixed)
            .add_option(open)
    }
}
