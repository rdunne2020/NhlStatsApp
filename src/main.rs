use std::str::FromStr;

use reqwest::{Error, Client, RequestBuilder};
//use serde::Deserialize;
use serde_json::{Value};
 
// Declare the module player
pub mod player;
pub mod teams;

#[tokio::main]
async fn main() -> Result<(), Error> {

    let get_client: Client = Client::new(); 

    let request_url = "https://statsapi.web.nhl.com/api/v1/teams?expand=team.roster";
    // Create a request using the get method, returns a RequestBuilder, then from there we send it
    let http_req: RequestBuilder = get_client.get(request_url);

    //let json_response = response.text().await?;
    let json_response = (http_req.send().await?)
                                                .text()
                                                .await?;

    // Pull the json data into the Value enum for indexing in
    let v: Value = serde_json::from_str(json_response.as_str()).unwrap();

    let mut all_teams: Vec<teams::Team> = Vec::new();

    for i in 0..teams::NUM_TEAMS {
        let mut new_team: teams::Team = Default::default();
        // The Value serde object has a Display impl that lets you format! the strings to get the values
        new_team.name = format!("{:#}", v["teams"][i]["name"]);
        new_team.abbrev = format!("{:#}", v["teams"][i]["abbreviation"]);
        let division_str = v["teams"][i]["division"]["name"].as_str().unwrap();
        new_team.division = teams::division_from_string(division_str).unwrap();
        new_team.id = v["teams"][i]["id"].as_u64().unwrap() as usize;

        // Get player data
        let roster_data = &v["teams"][i]["roster"]["roster"];
        // Get the number of players out to loop thru it
        let num_players = match roster_data {
            Value::Array(v) => v.len(),
            _ => 0
        };

        for j in 0..num_players {
            let player_name = roster_data[j]["person"]["fullName"].as_str().unwrap().to_string();
            
            // Sometimes this isn't in the player data. default jersey # to zero in that case
            let player_num = match roster_data[j]["jerseyNumber"].as_str() {
                Some(s) => s.to_string().parse::<u64>().unwrap(),
                None => 0
            };

            let player_id = roster_data[j]["person"]["id"].as_u64().unwrap();
            // The as_str or as_u64 are traits defined by value, allowing for easy getting out of enum mode
            let player_position = roster_data[j]["position"]["abbreviation"].as_str().unwrap();
            let player_position_type = roster_data[j]["position"]["type"].as_str().unwrap();

            let player_obj: player::Player = player::Player {
                name: player_name,
                number: player_num,
                id: player_id,
                position: player::Position::from_str(player_position).unwrap(),
                position_type: player::PositionType::from_str(player_position_type).unwrap()
            };

            println!("{}", player_obj);
            new_team.players.push(player_obj);
        }

        // Implemented the display trait for the team
        println!("{}", new_team);
        all_teams.push(new_team);
    }

    Ok(())
}
