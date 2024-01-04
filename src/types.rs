use serde::{Deserialize, Serialize};

/// languages supported by Steam store
#[derive(Default, Clone, Debug, Hash)]
pub enum Language {
    #[default]
    All,
    Arabic,
    Bulgarian,
    Schinese,
    Tchinese,
    Czech,
    Danish,
    Dutch,
    English,
    Finnish,
    French,
    German,
    Greek,
    Hungarian,
    Indonesian,
    Italian,
    Japanese,
    Koreana,
    Norwegian,
    Polish,
    Portuguese,
    Brazilian,
    Romanian,
    Russian,
    Spanish,
    Latam,
    Swedish,
    Thai,
    Turkish,
    Ukrainian,
    Vietnamese,
}

impl AsRef<str> for Language {
    fn as_ref(&self) -> &str {
        use Language::*;

        match &self {
            All => "all",
            Arabic => "arabic",
            Bulgarian => "bulgarian",
            Schinese => "schinese",
            Tchinese => "tchinese",
            Czech => "czech",
            Danish => "danish",
            Dutch => "dutch",
            English => "english",
            Finnish => "finnish",
            French => "french",
            German => "german",
            Greek => "greek",
            Hungarian => "hungarian",
            Indonesian => "indonesian",
            Italian => "italian",
            Japanese => "japanese",
            Koreana => "koreana",
            Norwegian => "norwegian",
            Polish => "polish",
            Portuguese => "portuguese",
            Brazilian => "brazilian",
            Romanian => "romanian",
            Russian => "russian",
            Spanish => "spanish",
            Latam => "latam",
            Swedish => "swedish",
            Thai => "thai",
            Turkish => "turkish",
            Ukrainian => "ukrainian",
            Vietnamese => "vietnamese",
        }
    }
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct Platforms {
    pub windows: bool,
    pub mac: bool,
    pub linux: bool,
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct ReleaseDate {
    pub coming_soon: Option<bool>,
    pub date: Option<String>,
    pub steam: Option<String>,
}
