use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for arg in std::env::args() {
        println!("{}", arg);
    }

    let url = "https://www.rust-lang.org/";
    let output = "rust.md";

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url)?.text()?;

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes())?;
    println!("Converted markdown has been saved in {}.", output);

    Ok(())

    // let alice = User { id: UserId(1), name: "Alice".into(), gender: Gender::Female };
    // let bob = User { id: UserId(2), name: "Bob".into(), gender: Gender::Male };
    // let topic = Topic { id: TopicId(1), name: "rust".into(), owner: UserId(1) };
    // let event1 = Event::Join((alice.id, topic.id));
    // let event2 = Event::Join((bob.id, topic.id));
    // let event3 = Event::Message((alice.id, topic.id, "Hello world".into()));
    //
    // println!("event1: {:?}, event2: {:?}, event3: {:?}", event1, event2, event3);
}

// #[derive(Debug)]
// enum Gender {
//     Unspecified = 0,
//     Female = 1,
//     Male = 2,
// }
//
// #[derive(Debug, Copy, Clone)]
// struct UserId(u64);
//
// #[derive(Debug, Copy, Clone)]
// struct TopicId(u64);
//
// #[derive(Debug)]
// struct User {
//     id: UserId,
//     name: String,
//     gender: Gender,
// }
//
// #[derive(Debug)]
// struct Topic {
//     id: TopicId,
//     name: String,
//     owner: UserId,
// }
//
// #[derive(Debug)]
// enum Event {
//     Join((UserId, TopicId)),
//     Leave((UserId, TopicId)),
//     Message((UserId, TopicId, String)),
// }
