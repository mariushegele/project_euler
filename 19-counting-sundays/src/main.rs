use counting_sundays::*;

fn main() {
    let mut range = DateRange::new(
        Date::new(1, Month::January, 1901),
        Date::new(1, Month::January, 2001)
    );
    let num = range.number_of_sundays_on_first_of_month();
    println!("The number of sundays on the first of the month during the twentieth century is {num}");
}
