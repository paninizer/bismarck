use poise;
use data::Data;

use crate::data;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type FrameworkError<'a> = poise::FrameworkError<'a, Data, Error>;