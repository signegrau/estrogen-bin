use chrono::prelude::*;
use chrono::{Duration, Local, TimeZone};

fn main() {
    let first_date = Local.ymd(2020, 10, 28);
    let now = Local::now().date();
    //let now = Local.ymd(2021, 11, 29);

    let duration = now - first_date;

    let weeks = duration.num_weeks();
    let days_rest = duration.num_days() % 7;

    if days_rest == 0 {
        println!("Begyndte på hormoner for {} uger siden 🥳", weeks);
    } else if days_rest == 1 {
        println!("Begyndte på hormoner for {} uger og 1 dag siden 🥳", weeks);
    } else {
        println!(
            "Begyndte på hormoner for {} uger og {} dage siden 🥳",
            weeks, days_rest
        );
    }

    let has_reached_date = now.day().checked_sub(first_date.day()).is_some();

    let months = (12
        + ((now.month() as i32)
            - (first_date.month() as i32)
            - if has_reached_date { 0 } else { 1 }))
        % 12;

    let years = now.year()
        - first_date.year()
        - if let Some(m) = now.month().checked_sub(first_date.month()) {
            if m > 0 || has_reached_date {
                0
            } else {
                1
            }
        } else {
            1
        };

    if years > 0 {
        if months == 1 {
            println!("Det er {} år og 1 hel måned! 🎉", years);
        } else if months != 0 {
            println!("Det er {} år og {} hele måneder! 🎉", years, months);
        } else {
            println!("Det er {} år! 🎉", years);
        }
    } else if months == 1 {
        println!("Det er 1 hel måned! 🎉");
    } else if months != 0 {
        println!("Det er {} hele måneder! 🎉", months);
    }

    let days_of_e_left = 50 - ((duration.num_days() + 1) % 50);
    let next_run = now + Duration::days(days_of_e_left);

    println!(
        "\nDer skulle gerne være {} piller østradiol tilbage, hvilket svarer til {} dage 💊",
        days_of_e_left * 2,
        days_of_e_left
    );

    println!(
        "Vi tager hul på nye forsyninger den {} 🗓",
        next_run.format_localized("%e. %B %Y", Locale::da_DK)
    )
}
