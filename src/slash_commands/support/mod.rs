mod get;
mod list;

use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    Permissions, ResolvedOption, ResolvedValue,
};
use sqlx::{Database, Pool};
use zayden_core::parse_options;

use crate::{Error, Result, TicketGuildManager};

pub struct SupportCommand;

impl SupportCommand {
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
            "get" => {
                Self::get::<Db, GuildManager>(ctx, interaction, pool, options, guild_id).await?
            }
            "list" => Self::list::<Db, GuildManager>(ctx, interaction, pool, guild_id).await?,
            _ => unreachable!("Subcommand is required"),
        }

        Ok(())
    }

    pub fn register() -> CreateCommand {
        let list = CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "list",
            "List all support messages",
        );
        let get = CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "get",
            "Get a support message",
        )
        .add_sub_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "id",
                "The ID of the support message",
            )
            .required(true),
        );

        CreateCommand::new("support")
            .description("Support FAQ commands")
            .default_member_permissions(Permissions::MANAGE_MESSAGES)
            .add_option(get)
            .add_option(list)
    }
}
