use chrono::*;
use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LastGames {
    pub games: Vec<LastGame>,
}

#[derive(Deserialize, Debug)]
pub struct LastGame {
    #[serde(rename(deserialize = "gameId"))]
    pub game_id: String,
    pub arena: Arena,
    #[serde(rename(deserialize = "statusNum"))]
    pub status_num: u8,
    #[serde(rename(deserialize = "hTeam"))]
    pub home_team: Team,
    #[serde(rename(deserialize = "vTeam"))]
    pub visitor_team: Team,
}

#[derive(Deserialize, Debug)]
pub struct Team {
    #[serde(rename(deserialize = "teamId"))]
    pub team_id: String,
    #[serde(rename(deserialize = "triCode"))]
    pub tri_code: String,
    pub win: String,
    pub loss: String,
    pub score: String,
    #[serde(rename(deserialize = "linescore"))]
    pub line_score: Vec<LineScore>,
}

impl LastGames {
    pub fn get_last_games() -> Result<Vec<LastGame>, reqwest::Error> {
        let yesterday: DateTime<Local> = Local::now() - Duration::days(1);
        let yesterday_in_str = yesterday.format("%Y%m%d");
        let base_url = "http://data.nba.net/prod/v1";
        let path_url = "scoreboard.json";
        let endpoint = format!("{}/{}/{}", base_url, yesterday_in_str, path_url);
        let result: LastGames = reqwest::blocking::get(endpoint)?.json()?;
        //println!("deserialized = {:?}", result.games[0]);
        Ok(result.games)
    }
}

#[derive(Deserialize, Debug)]
pub struct Arena {
    pub name: String,
    pub city: String,
    #[serde(rename(deserialize = "stateAbbr"))]
    pub state_abbr: String,
}

#[derive(Deserialize, Debug)]
pub struct LineScore {
    pub score: String,
}
