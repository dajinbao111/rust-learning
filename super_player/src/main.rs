use crate::media::Playable;
use crate::trait_inheritance::{Car, Vehicle};

mod media;
mod trait_inheritance;

struct Audio(String);
struct Video(String);

impl Playable for Audio {
    fn play(&self) {
        println!("playing: {}", self.0);
    }
}

impl Playable for Video {
    fn play(&self) {
        println!("playing: {}", self.0);
    }
}

fn main() {
    println!("Super player!");
    let audio = Audio("ambient_music.mp3".to_string());
    let video = Video("big_buck.mkv".to_string());
    audio.play();
    video.play();

    let my_roadster = crate::trait_inheritance::TeslaRoadster::new("Tesla Roadster II", 2020);
    println!("{} is priced at ${}", my_roadster.model, my_roadster.get_price());
}
