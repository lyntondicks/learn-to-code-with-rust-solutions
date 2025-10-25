#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

pub fn main() {
    println!("--- Chapter 10: Enums --");
    enums();
    chapter_10_enums_project_solution();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

#[derive(Debug)]
struct Card {
    suite: CardSuit,
    rank: u8, // 1-13 for Ace to King
}

#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal { username: String, password: String },
    Cash,
}

#[derive(Debug)]
enum Beans {
    Pinto,
    Black,
    Kidney,
}

#[derive(Debug)]
enum Meat {
    Chicken,
    Beef,
    Pork,
    Lamb,
}

#[derive(Debug)]
enum RestaurantItem {
    Burrito { meat: Meat, beans: Beans },
    Bowl { meat: Meat, beans: Beans },
    VeganPlate,
}

#[derive(Debug)]
enum OperatingSystem {
    Windows,
    MacOS,
    Linux,
}

fn years_since_release(os: OperatingSystem) -> u32 {
    match os {
        OperatingSystem::Windows => {
            println!("Windows operating system selected.");
            37
        }
        OperatingSystem::MacOS => 39,
        OperatingSystem::Linux => 32,
    }
}

enum LaundryCycle {
    Cold,
    Hot { temperature: u32 },
    Delicate(String),
}

impl LaundryCycle {
    fn wash_laundry(&self) {
        match self {
            LaundryCycle::Cold => {
                println!("Running the laundry with cold temperature")
            }
            LaundryCycle::Hot { temperature } => {
                println!(
                    "Running the laundry with hot temperature: {}°C",
                    temperature
                )
            }
            LaundryCycle::Delicate(item) => {
                println!("Running the delicate cycle for: {}", item)
            }
        }
    }
}

#[derive(Debug)]
enum OnlineOrderStatus {
    Ordered,
    Packed,
    Shipped,
    Delivered,
}

impl OnlineOrderStatus {
    fn check(&self) {
        match self {
            OnlineOrderStatus::Ordered | OnlineOrderStatus::Packed => {
                println!("Your order is being processed.")
            }
            OnlineOrderStatus::Delivered => {
                println!("Your order has been delivered!")
            }
            other_status => {
                println!("Your item is {:?}", other_status)
            } // _ => {
              //     println!("Your order is still in progress.")
              // }
        }
    }
}

#[derive(Debug)]
enum Milk {
    Lowfat(isize),
    Whole,
    NonDairy { kind: String },
}

impl Milk {
    fn drink(self) {
        match self {
            Milk::Lowfat(2) => {
                println!("2% milk is the course author's favorite.");
            }
            Milk::Lowfat(amount) => {
                println!("Drinking {}% milk.", amount);
            }
            Milk::Whole => {
                println!("Drinking whole milk.");
            }
            Milk::NonDairy { kind } => {
                println!("Drinking non-dairy milk made from {}", kind);
            }
        }
    }
}

fn enums() {
    let first_card = Card {
        suite: CardSuit::Hearts,
        rank: 1,
    };

    let mut second_card = Card {
        suite: CardSuit::Spades,
        rank: 13,
    };
    second_card.suite = CardSuit::Clubs;
    println!(
        "First card: {:?}, Second card: {:?}",
        first_card, second_card
    );

    let card_suits = [CardSuit::Hearts, CardSuit::Clubs];
    let card_suits = (CardSuit::Hearts, CardSuit::Spades);

    let visa = PaymentMethodType::CreditCard(String::from("1234-5678-9012-3456"));
    let mastercard = PaymentMethodType::DebitCard(String::from("9876-5432-1098-7654"));
    println!("{:?}", visa);
    println!("{:?}", mastercard);

    let mut my_payment_method = PaymentMethodType::CreditCard(String::from("1234-5678-9012-3456"));
    my_payment_method = PaymentMethodType::PayPal {
        username: String::from("user@example.com"),
        password: String::from("password"),
    };
    println!("{:?}", my_payment_method);

    let lunch = RestaurantItem::Burrito {
        meat: Meat::Beef,
        beans: Beans::Black,
    };
    let dinner = RestaurantItem::Bowl {
        meat: Meat::Chicken,
        beans: Beans::Pinto,
    };
    let abandoned_meal = RestaurantItem::VeganPlate;
    println!(
        "Lunch was {lunch:?} and dinner was {dinner:?} and abandoned meal was {abandoned_meal:?}"
    );

    let my_computer = OperatingSystem::Linux;
    let age = years_since_release(my_computer);
    println!("My computer's operating system is {} years old.", age);

    LaundryCycle::Cold.wash_laundry();
    LaundryCycle::Hot { temperature: 60 }.wash_laundry();
    LaundryCycle::Delicate(String::from("silk shirt")).wash_laundry();

    OnlineOrderStatus::Shipped.check();

    Milk::Lowfat(1).drink();
    Milk::Lowfat(2).drink();
    Milk::Whole.drink();

    let my_beverage = Milk::Lowfat(2);
    if let Milk::Lowfat(percent) = my_beverage {
        println!("Drinking {}% milk.", percent);
    }

    if let Milk::NonDairy { kind } = &my_beverage {
        println!("Drinking non-dairy milk made from {}", kind);
    }

    let Milk::Lowfat(percent) = &my_beverage else {
        println!("Not drinking low-fat milk.");
        return; // terminate the function because we're not drinking low-fat milk
    };

    println!("Drinking {}% milk.", percent); // lowfat at this point
}

// Chapter 10 Enums Project Solution
#[derive(Debug)]
enum Tier {
    Gold,
    Silver,
    Platinum,
}

enum Subscription {
    Free,
    Basic(f64, u32), // (price per month, duration in months)
    Premium { tier: Tier },
}

impl Subscription {
    fn summarize(&self) {
        match self {
            Subscription::Free => {
                println!("You have limited access to the site");
            }
            Subscription::Basic(price, duration) => {
                println!(
                    "You have limited access to the site's premium features for ${} per month for {} months",
                    price, duration
                );
            }
            Subscription::Premium { tier } => {
                println!(
                    "You have full access to the site's premium features. Your tier is: {:?}",
                    tier
                );
            }
        }
    }
}

fn chapter_10_enums_project_solution() {
    Subscription::Free.summarize();

    let basic = Subscription::Basic(4.99, 3);
    basic.summarize();

    let premium = Subscription::Premium {
        tier: Tier::Platinum,
    };
    premium.summarize();
}
