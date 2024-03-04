use std::collections::HashMap;
use regex::Regex;
use serde::{Deserialize, Serialize};

const API: &str = "https://lab.magiconch.com/api/nbnhhsh";

pub type GuessResponseRoot = Vec<GuessResponse>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuessResponse {
    pub name: String,
    #[serde(default)]
    pub trans: Vec<String>,
    #[serde(default)]
    pub inputting: Vec<String>,
}

pub async fn guess(text: String) -> Result<GuessResponseRoot, reqwest::Error> {
    let re = Regex::new(r"[a-zA-Z0-9]+").unwrap();
    let words: Vec<&str> = re.captures_iter(&*text).filter_map(|x| x.get(0)).map(|x| x.as_str()).collect();

    let mut body = HashMap::new();
    body.insert("text", words.join(","));
    let client = reqwest::Client::new();
    let resp = client.post(API.to_owned() + "/guess")
        .json(&body)
        .send()
        .await?
        .json::<GuessResponseRoot>()
        .await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_guess() {
        let fbi = guess("fbi".to_string()).await;
        assert!(fbi.is_ok(), "should ok");
        assert!(fbi.is_ok_and(|x| x[0].trans[0] == "美国联邦调查局"), "should be fbi");
    }
}