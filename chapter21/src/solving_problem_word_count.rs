use std::collections::HashMap;

fn count_words(text: &str) -> HashMap<&str, usize> {
    let words = text.split_whitespace();
    let mut counts = HashMap::new();
    for word in words {
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }
    counts
}

fn count_characters(text: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for character in text.chars() {
        let count = counts.entry(character).or_insert(0);
        *count += 1;
    }
    counts
}

fn count_characters_for_each(text: &str) -> HashMap<char, usize> {
    let words = text.split_whitespace();
    let mut counts = HashMap::new();

    words.for_each(|word| {
        word.chars().for_each(|character| {
            let count = counts.entry(character).or_insert(0);
            *count += 1;
        })
    });

    counts
}

pub fn solving_problem_word_count() {
    println!("Chapter 21: Solving Problem Word Count");

    let text = "Sally sells sea shells by the sea shore";
    println!("{:?}", count_words(text));
    println!("{:?}", count_characters(text));
    println!("for_each {:?}", count_characters_for_each(text));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_correct_word_count() {
        // Arrange
        let text = "Sally sells sea shells by the sea shore.";
        let expected_word_count = HashMap::from([
            ("Sally", 1),
            ("sells", 1),
            ("sea", 2),
            ("shells", 1),
            ("by", 1),
            ("the", 1),
            ("shore.", 1),
        ]);

        // Act
        let result = count_words(text);

        // Assert
        assert_eq!(result, expected_word_count);
    }

    #[test]
    fn should_have_correct_character_count() {
        // Arrange
        let text = "Sally sells sea shells by the sea shore.";
        let expected_character_count = HashMap::from([
            ('S', 1),
            ('a', 3),
            ('l', 6),
            ('y', 2),
            (' ', 7),
            ('s', 7),
            ('e', 6),
            ('h', 3),
            ('b', 1),
            ('t', 1),
            ('o', 1),
            ('r', 1),
            ('.', 1),
        ]);

        // Act
        let result = count_characters(text);

        // Assert
        assert_eq!(result, expected_character_count);
    }
}
