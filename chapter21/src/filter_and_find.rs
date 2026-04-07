pub fn filter_and_find() {
    println!("Chapter 21: Filter and Find");
    filter_and_find_simple_types();
    filter_and_find_complex_types();
}

pub fn filter_and_find_simple_types() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let evens: Vec<i32> = numbers.iter().filter(|x| *x % 2 == 0).copied().collect();
    println!("Even numbers: {:?}", evens);
    println!("Numbers: {:?}", numbers);

    let first_even = numbers.into_iter().find(|x| x % 2 == 0);
    println!("First even number: {:?}", first_even);
    println!("Numbers: {:?}", numbers); // numbers is still usable here because [i32] implements Copy

    let first_odd = numbers.into_iter().find(|x| x % 2 != 0);
    println!("First odd number: {:?}", first_odd);

    let nothing = numbers.into_iter().find(|x| *x > 100);
    println!("Nothing: {:?}", nothing);

    let last_even = numbers.into_iter().rfind(|x| x % 2 == 0);
    println!("Last even number: {:?}", last_even);
}

#[derive(Debug, PartialEq, Eq)]
enum ChannelType {
    Comedy,
    News,
    ProgrammingTutorials,
}

#[derive(Debug)]
struct TvChannel {
    name: String,
    channel_type: ChannelType,
}

pub fn filter_and_find_complex_types() {
    let channels = [
        TvChannel {
            name: String::from("CBS"),
            channel_type: ChannelType::Comedy,
        },
        TvChannel {
            name: String::from("RustLive"),
            channel_type: ChannelType::ProgrammingTutorials,
        },
        TvChannel {
            name: String::from("NBC"),
            channel_type: ChannelType::News,
        },
        TvChannel {
            name: String::from("RustTV"),
            channel_type: ChannelType::ProgrammingTutorials,
        },
    ];

    let good_channels: Vec<&str> = channels
        .iter()
        .filter(|channel| channel.channel_type == ChannelType::ProgrammingTutorials)
        .map(|channel| channel.name.as_str())
        .collect();
    println!("Good channels: {:?}", good_channels);

    let good_channel = channels
        .iter()
        .find(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);

    match good_channel {
        Some(channel) => println!("First good channel: {:?}", channel),
        None => println!("No good channels found"),
    }
}
