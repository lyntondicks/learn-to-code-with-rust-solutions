use std::io::stdin;

#[derive(Debug)]
struct Vault {
    password: String,
    treasure: String,
}

impl Vault {
    /// Unlocks the vault using the provided procedure and returns the treasure if the password matches.
    ///
    /// # Arguments
    ///
    /// * `procedure` - A closure that returns the user's password.
    ///
    /// # Returns
    ///
    /// * `Option<String>` - The treasure if the password matches, otherwise `None`.
    fn unlock<F>(self, procedure: F) -> Option<String>
    where
        F: FnOnce() -> String,
    {
        let user_password = procedure();
        if user_password == self.password {
            Some(self.treasure)
        } else {
            None
        }
    }
}

pub fn fn_once_trait() {
    println!("Chapter 20: FnOnce Trait");
    let vault = Vault {
        password: String::from("topsecret"),
        treasure: String::from("The treasure is 3 million dollars!"),
    };

    let hack = || {
        println!("Please enter the vault password:");
        let mut user_input = String::new();
        stdin().read_line(&mut user_input).unwrap();
        user_input.trim().to_string()
    };

    let extraction = vault.unlock(hack);
    println!("Vault opening result: {:?}", extraction);

    let mut game_console = String::from("PlayStation");
    let mut deleted_characters = String::new();

    let closure = |character| {
        let is_not_a = character != 'a';
        if is_not_a {
            true
        } else {
            deleted_characters.push(character); // Changes type to FnMut because it borrows a mutable reference to deleted_characters
            false
        }
    };

    game_console.retain(closure);
    println!("Modified game console name: {}", game_console);
    println!("Deleted characters: {}", deleted_characters);
}
