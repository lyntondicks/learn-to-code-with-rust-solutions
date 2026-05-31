pub fn filter_map() {
    println!("Chapter 21: Filter Map");

    let stocks = ["nvda", "", "amd", "msft", "", "tsla", "aapl"];
    let capitalized_stocks = stocks
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_uppercase())
        .collect::<Vec<String>>();
    println!("Capitalized stocks: {:?}", capitalized_stocks);

    let capitalized_stocks = stocks
        .iter()
        .filter_map(|s| {
            if s.is_empty() {
                None
            } else {
                Some(s.to_uppercase())
            }
        })
        .collect::<Vec<String>>();
    println!(
        "Capitalized stocks using filter_map: {:?}",
        capitalized_stocks
    );
}
