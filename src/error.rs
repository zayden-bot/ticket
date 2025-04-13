use serenity::all::DiscordJsonError;
use serenity::all::HttpError;
use zayden_core::Error as ZaydenError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UnknownInteraction(serenity::Error),
    MissingGuildId,
    NotInSupportChannel,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::UnknownInteraction(_) => ZaydenError::UnknownInteraction.fmt(f),
            Error::MissingGuildId => ZaydenError::MissingGuildId.fmt(f),
            Error::NotInSupportChannel => {
                write!(f, "This command only works in the support channel.")
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<serenity::Error> for Error {
    fn from(e: serenity::Error) -> Self {
        match e {
            serenity::Error::Http(HttpError::UnsuccessfulRequest(
                serenity::all::ErrorResponse {
                    error: DiscordJsonError { code: 10062, .. },
                    ..
                },
            )) => Error::UnknownInteraction(e),
            _ => panic!("Unhandled Serenity error: {:?}", e),
        }
    }
}
