
use std::fmt;
use serde::{Deserialize, Serialize};
use serde_json;



#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    competition: serde_json::Value,
    pub filters: Filter,
    matches: Vec<serde_json::Value>,
    resultSet: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filter {
    season: String,
}


#[derive(Debug, Serialize)]
pub struct MatchInfo {
    date: String,
    home_team: String,
    away_team: String,
    status: String,
    score: String,
}

impl From<&serde_json::Value> for MatchInfo {
    fn from(match_obj: &serde_json::Value) -> Self {
        let date = match_obj.get("utcDate").and_then(|d| d.as_str()).unwrap_or("").to_string();
        let home_team = match_obj.get("homeTeam").and_then(|t| t.get("name")).and_then(|n| n.as_str()).unwrap_or("").to_string();
        let away_team = match_obj.get("awayTeam").and_then(|t| t.get("name")).and_then(|n| n.as_str()).unwrap_or("").to_string();
        let status = match_obj.get("status").and_then(|s| s.as_str()).unwrap_or("").to_string();
        let score = match_obj.get("score").and_then(|s| s.get("fullTime")).and_then(|ft| {
            let home_score = ft.get("home").and_then(|s| s.as_u64()).unwrap_or(0);
            let away_score = ft.get("away").and_then(|s| s.as_u64()).unwrap_or(0);
            Some(format!("{} - {}", home_score, away_score))
        }).unwrap_or_else(|| "".to_string());

        Self { date, home_team, away_team, status, score }
    }
}

impl Data {
    pub fn get_filters(&self) -> &Filter {
        &self.filters
    }
    pub fn get_match_info(&self) -> Vec<MatchInfo> {
        self.matches.iter().map(|match_obj| MatchInfo::from(match_obj)).collect()
    }
}





impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", "season",self.season)
    }
}



use serde_json::{Result, Value};
// use crate::Data;






// fn get_match_info_from(match_obj: &serde_json::Value) -> MatchInfo {
//     let date = match_obj.get("utcDate").and_then(|d| d.as_str()).unwrap_or("").to_string();
//     let home_team = match_obj.get("homeTeam").and_then(|t| t.get("name")).and_then(|n| n.as_str()).unwrap_or("").to_string();
//     let away_team = match_obj.get("awayTeam").and_then(|t| t.get("name")).and_then(|n| n.as_str()).unwrap_or("").to_string();
//     let status = match_obj.get("status").and_then(|s| s.as_str()).unwrap_or("").to_string();
//     let score = match_obj.get("score").and_then(|s| s.get("fullTime")).and_then(|ft| {
//         let home_score = ft.get("home").and_then(|s| s.as_u64()).unwrap_or(0);
//         let away_score = ft.get("away").and_then(|s| s.as_u64()).unwrap_or(0);
//         Some(format!("{} - {}", home_score, away_score))
//     }).unwrap_or_else(|| "".to_string());

//     MatchInfo { date, home_team, away_team, status, score }
// }

// impl Data {
//     pub fn get_filters(&self) -> &Filter {
//         &self.filters
//     }

//     pub fn get_match_info(&self) -> Vec<MatchInfo> {
//         self.matches.iter().map(|match_obj| get_match_info_from(match_obj)).collect()
//     }
// }
