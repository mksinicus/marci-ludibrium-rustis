use serde_yaml;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut firsts: HashMap<i32, i32> = HashMap::new();
    // firsts.insert(2001, 1);
    // println!("{}", yaml);
    // let mut year = String::new();
    // io::stdin().read_line(&mut year).unwrap();
    // println!("{}", first_weekday_by_mod(year.trim().parse().unwrap()));
    for year in 1..3000 {
        firsts.insert(year, first_weekday(year));
    }
    let yaml = serde_yaml::to_string(&firsts).unwrap();
    let mut file = File::create("first_weekdays.yaml").expect("Err: File creation failure");
    writeln!(&mut file, "{}", yaml).expect("Fuck you error");
    ()
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn first_weekday_by_sequence(year: i32) -> i32 {
    1
}

fn first_weekday(year: i32) -> i32 {
    // 2001-01-01 is Monday.
    let diff = year - 2001;
    if diff == 0 {
        return 1;
    }

    let plus = diff.is_positive();
    let mut leap_years: i32 = 0;
    let years = if plus { 2001..year } else { year..2001 };
    for a_year in years {
        if is_leap_year(a_year) {
            leap_years += 1;
        }
    }
    if !plus {
        leap_years = leap_years.checked_neg().unwrap()
    }
    let days = 365 * diff + leap_years;
    let result = (1 + days) % 7;

    // yields result
    if result.is_negative() {
        result + 7
    } else {
        result
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0) && ((year % 400 == 0) || (year % 100 != 0))
}
