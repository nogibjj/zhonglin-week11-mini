use rand::seq::SliceRandom;

fn main() {
    let headlines = vec![
        "Trump Sued for Fraud by New York Attorney General",
        "Lawsuit Accuses Trump of Misusing Campaign Funds",
        "Trump Faces Lawsuit Over Alleged Sexual Assault",
        "Trump University Settles Fraud Lawsuit for $25 Million",
        "Trump Foundation Accused of Misusing Charitable Funds",
    ];

    let quotes = vec![
        "I have never engaged in any fraudulent activity.",
        "This lawsuit is politically motivated and has no merit.",
        "I will vigorously defend myself against these baseless accusations.",
        "The mainstream media is out to get me and will stop at nothing to destroy my reputation.",
        "I am confident that the courts will rule in my favor.",
    ];

    let mut rng = rand::thread_rng();
    let headline = headlines.choose(&mut rng).unwrap();
    let quote = quotes.choose(&mut rng).unwrap();

    println!("{}", headline);
    println!("");
    println!("In a breaking news story, former President Donald Trump has been sued by a New York attorney general for alleged fraud. According to the lawsuit, Trump is accused of misusing campaign funds and engaging in fraudulent activity during his time in office.");
    println!("");
    println!("Trump has denied these allegations, stating: '{}'", quote);
    println!("");
    println!("This is a developing story and we will provide updates as more information becomes available.");
}
