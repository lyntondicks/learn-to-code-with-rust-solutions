use std::{collections::HashSet, ops::RangeInclusive};

#[allow(unused)]
#[derive(Debug)]
struct PlayList {
    songs: Vec<String>,
    users: HashSet<String>,
}

impl FromIterator<(String, String)> for PlayList {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
        let mut songs = Vec::new();
        let mut users = HashSet::new();
        for (song, user) in iter {
            songs.push(song);
            users.insert(user);
        }
        Self { songs, users }
    }
}

impl<'a> FromIterator<&'a (String, String)> for PlayList {
    fn from_iter<T: IntoIterator<Item = &'a (String, String)>>(iter: T) -> Self {
        let mut songs = Vec::new();
        let mut users = HashSet::new();
        for (song, user) in iter {
            songs.push(String::from(song));
            users.insert(String::from(user));
        }
        Self { songs, users }
    }
}

pub fn main() {
    let fifty_numbers: RangeInclusive<i32> = 1..=50;
    let results = Vec::from_iter(fifty_numbers.clone());
    println!("{results:#?}");

    let results = fifty_numbers.clone().collect::<Vec<i32>>(); // .collect() uses from_iter
    println!("{results:#?}");

    let unique_set: HashSet<_> = HashSet::from_iter(fifty_numbers.clone());
    println!("{unique_set:#?}");

    let unique_set = fifty_numbers.clone().collect::<HashSet<i32>>(); // .collect() uses from_iter
    println!("{unique_set:#?}");

    let chars = ['H', 'e', 'l', 'l', 'o']; // implements IntoIterator
    let greeting = String::from_iter(chars); // from_iter uses IntoIterator trait to get an iterator
    println!("{greeting}");

    let songs = [
        (String::from("I Rust Go On"), String::from("Bob")),
        (String::from("A Rust Of Wind"), String::from("Bob")),
        (String::from("A Rustworthy Man"), String::from("Sheila")),
    ];

    let playlist = PlayList::from_iter(songs.clone());
    println!("{playlist:#?}");

    let playlist_collect = songs.clone().into_iter().collect::<PlayList>();
    println!("{playlist_collect:#?}");

    let ref_playlist_collect = songs.iter().collect::<PlayList>();
    println!("{ref_playlist_collect:#?}");
}
