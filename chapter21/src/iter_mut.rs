pub fn iter_mut() {
    println!("Chapter 21: iter_mut");

    let mut flavors = [
        String::from("Chocolate"),
        String::from("Vanilla"),
        String::from("Strawberry"),
    ];

    // let iterator = flavors.iter_mut();
    // for flavor in iterator {
    //   flavor.push_str(" Ice cream");
    // }

    for flavor in &mut flavors {
        // for loop with mutable ref borrow will automatically call .iter_mut()
        flavor.push_str(" Ice cream");
    }

    println!("{flavors:?}");

    let mut school_grades = [24, 57, 78];
    for grade in &mut school_grades {
        *grade -= 2; // dereference
    }
    println!("{school_grades:?}");
}

/*
Summary:

OWNERSHIP
for value in collection
for value in collection.into_iter()

IMMUTABLE REFERENCES
for value in &collection
for value in collection.iter()

MUTABLE REFERENCES
for value in &mut collection
for value in collection.iter_mut()

*/
