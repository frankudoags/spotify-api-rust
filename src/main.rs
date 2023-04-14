use reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct ExternalUrls {
    spotify: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Artist {
    name: String,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Album {
    name: String,
    artists: Vec<Artist>,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Track {
    name: String,
    href: String,
    popularity: u32,
    album: Album,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Items<T> {
    items: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    tracks: Items<Track>,
}

fn print_tracks(tracks: Vec<&Track>) {
    for track in tracks {
        println!("{}", track.name);
        println!("{}", track.album.name);
        println!(
            "{}",
            track
                .album
                .artists
                .iter()
                .map(|artist| artist.name.to_string())
                .collect::<String>()
        );
        println!("{}", track.external_urls.spotify);
        println!("---------")
    }
}

#[tokio::main]
async fn main() {
    // grab the search query and auth token from the command line
    let args: Vec<String> = env::args().collect();
    // check that we have the right number of args
    if args.len() != 3 {
        eprintln!("WRONG USAGE   Usage: cargo run -- <search query> <auth token>");
        std::process::exit(1);
    }

    //extract the search query and auth token from the args vector
    let search_query = &args[1];
    // Here's a sample auth token. You can get your own by following the instructions here:
    // https://developer.spotify.com/documentation/general/guides/authorization-guide/

    // BQD9i54YF339wP2LoKTQQUbfGVCczvpT0VT5tDEHl63CJkRS5gihLYJCSlYQwU95FhvO5sV1B_ogxVJeAirPlmo_6umKXSJCPxUb2-ZDKn8XtilhBqGhx6HzCs3TLEZv2hvPjStR7aq4gslRV0yzvxHEDSKUBK6kIipxH73FRas1CkPMFSeQ2U71v4aoHBR1bCwGSIQedaknoMMWbKuEgbjgFszXJ0i7GTRdAdkNhRsUubgUI9mldnxMgeJQZW8NYoe-1XMASHRH87wq0_5f_p90beyac74Az3BXZ9KbNqfLUqCMLW6PSxJ7c6TtZrdEyUniRBIKUMGPpeaoDnRKuM15LR2ShE3z1Yl1mplFABY
    let auth_token = &args[2];
    let url = format!(
        "https://api.spotify.com/v1/search?q={query}&type=track,artist",
        query = search_query
    );
    // create a client from the reqwest crate
    let client = reqwest::Client::new();
    // make a request to the Spotify API
    let response = client
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {}", auth_token))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();
    // match on the response status
    match response.status() {
        // if the status is 200(OK), we're good to go
        reqwest::StatusCode::OK => {
            match response.json::<APIResponse>().await {
                Ok(parsed) => print_tracks(parsed.tracks.items.iter().collect()),
                Err(_) => println!("Hm, the response didn't match the shape we expected."),
            };
        }
        // if the status is 401(Unauthorized), we need to grab a new token
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token");
        }
        // if the status is anything else, we need to panic and figure out what's going on
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    };
}