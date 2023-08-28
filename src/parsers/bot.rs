use anyhow::Result;

use serde::Deserialize;

use lazy_static::lazy_static;
use serde::Serialize;

use crate::parsers::utils::user_agent_match;
use crate::parsers::utils::LazyRegex;

lazy_static! {
    static ref BOT_LIST: BotList = {
        let contents = include_str!("../../regexes/bots.yml");
        BotList::from_file(contents).unwrap_or_else(|_| panic!("loading bots.yml"))
    };
}

pub fn lookup_bot(ua: &str) -> Result<Option<Bot>> {
    BOT_LIST.lookup(ua)
}

#[derive(Debug, Serialize)]
pub struct Bot {
    pub name: String,
    pub category: Option<String>,
    pub url: Option<String>,
    pub producer: Option<BotProducer>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct BotProducer {
    pub name: String,
    pub url: String,
}

#[derive(Debug)]
struct BotEntry {
    regex: LazyRegex,
    name: String,
    url: Option<String>,
    category: Option<String>,
    producer: Option<BotProducer>,
}

impl From<&BotEntry> for Bot {
    fn from(entry: &BotEntry) -> Self {
        Bot {
            name: entry.name.clone(),
            category: entry.category.clone(),
            url: entry.url.clone(),
            producer: entry.producer.clone(),
        }
    }
}

#[derive(Debug)]
pub struct BotList {
    bots: Vec<BotEntry>,
}

impl BotList {
    pub fn from_file(contents: &str) -> Result<BotList> {
        #[derive(Debug, Deserialize)]
        struct YamlBotEntry {
            regex: String,
            name: String,
            url: Option<String>,
            category: Option<String>,
            producer: Option<BotProducer>,
        }

        #[allow(clippy::from_over_into)]
        impl Into<BotEntry> for YamlBotEntry {
            fn into(self) -> BotEntry {
                BotEntry {
                    regex: user_agent_match(&self.regex),
                    name: self.name,
                    url: self.url,
                    category: self.category,
                    producer: self.producer,
                }
            }
        }

        #[derive(Debug, Deserialize)]
        #[serde(transparent)]
        struct YamlBotList {
            bots: Vec<YamlBotEntry>,
        }

        #[allow(clippy::from_over_into)]
        impl Into<BotList> for YamlBotList {
            fn into(self) -> BotList {
                BotList {
                    bots: self.bots.into_iter().map(|x| x.into()).collect(),
                }
            }
        }

        let res: YamlBotList = serde_yaml::from_str(contents)?;
        Ok(res.into())
    }

    fn lookup(&self, ua: &str) -> Result<Option<Bot>> {
        for bot in self.bots.iter() {
            if bot.regex.is_match(ua)? {
                return Ok(Some(bot.into()));
            }
        }

        Ok(None)
    }
}
