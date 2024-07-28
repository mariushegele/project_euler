type Year = i32;

fn is_leap_year(year: Year) -> bool {
    if year % 100 == 0 {
        return year % 400 == 0;
    }
    year % 4 == 0
}

#[derive(PartialEq, Debug)]
#[repr(u8)]
pub enum Weekday {
    Monday = 0,
    Tuesday = 1,
    Wednesday = 2,
    Thursday = 3,
    Friday = 4,
    Saturday = 5,
    Sunday = 6
}

impl Weekday {
    fn next(&self) -> Weekday {
        match *self {
            Weekday::Monday => Weekday::Tuesday,
            Weekday::Tuesday => Weekday::Wednesday,
            Weekday::Wednesday => Weekday::Thursday,
            Weekday::Thursday => Weekday::Friday,
            Weekday::Friday => Weekday::Saturday,
            Weekday::Saturday => Weekday::Sunday,
            Weekday::Sunday => Weekday::Monday,
        }
    }
}

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
#[repr(u8)]
pub enum Month {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12
}

impl Month {
    fn days_in_month(&self, year: Year) -> u8 {
        let leap_year = is_leap_year(year);
        match *self {
            Month::January => 31,
            Month::February if leap_year => 29,
            Month::February => 28,
            Month::March => 31,
            Month::April => 30,
            Month::May => 31,
            Month::June => 30,
            Month::July => 31,
            Month::August => 31,
            Month::September => 30,
            Month::October => 31,
            Month::November => 30,
            Month::December => 31,
        }
    }

    fn next(&self) -> Month {
        match *self {
            Month::January   => Month::February,
            Month::February  => Month::March,
            Month::March     => Month::April,
            Month::April     => Month::May,
            Month::May       => Month::June,
            Month::June      => Month::July,
            Month::July      => Month::August,
            Month::August    => Month::September,
            Month::September => Month::October,
            Month::October   => Month::November,
            Month::November  => Month::December,
            Month::December  => Month::January,
        }
    }
}

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub struct Date {
    year: Year,
    month_in_year: Month,
    day_in_month: u8,
}

impl Date {
    pub fn new(day_in_month: u8, month_in_year: Month, year: Year) -> Self {
        assert!(1 <= day_in_month 
            && day_in_month <= 31 
            && Month::January <= month_in_year
            && month_in_year <= Month::December, 
            "invalid date: {year}-{month_in_year:?}-{day_in_month}");
        
        Self {day_in_month, month_in_year, year}
    }

    pub fn add_one_day(&self) -> Self {
        let last_day_in_month = self.month_in_year.days_in_month(self.year);
        if self.day_in_month < last_day_in_month {
            Date::new(self.day_in_month + 1, self.month_in_year, self.year)
        } else if self.month_in_year == Month::December {
            Date::new(1, self.month_in_year.next(), self.year + 1)
        } else {
            Date::new(1, self.month_in_year.next(), self.year)
        }
    }

    pub fn date_range(start: Self, end: Self) -> DateRange {
        DateRange{start, end}
    } 
}

pub struct DateRange {
    start: Date,
    end: Date, 
}

impl DateRange {
    pub fn new(start: Date, end: Date) -> Self {
        Self {start, end}
    }

    pub fn number_of_sundays_on_first_of_month(&mut self) -> u32 {
        let epoch = Date::new(1, Month::January, 1900);
        let mut weekday = Weekday::Monday; 

        let pre_range_time = Self::new(epoch, self.start);
        for _ in pre_range_time {
            weekday = weekday.next();
        }

        let mut sundays_on_first_of_month = 0;
        for day in self {
            if weekday == Weekday::Sunday && day.day_in_month == 1 {
                sundays_on_first_of_month += 1;
            }
            weekday = weekday.next();
        }

        sundays_on_first_of_month
    }
}

impl Iterator for DateRange {
    type Item = Date;

    fn next(&mut self) -> Option<Date> {
        if self.start >= self.end {
            None
        } else {
            let result = Some(self.start);
            self.start = self.start.add_one_day();
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use std::iter;

    use super::*;

    #[test]
    fn test_is_leap_year() {
        assert_eq!(is_leap_year(1900), false); // century
        assert_eq!(is_leap_year(1901), false);
        assert_eq!(is_leap_year(1902), false);
        assert_eq!(is_leap_year(1903), false);
        assert_eq!(is_leap_year(1904), true);
        assert_eq!(is_leap_year(1905), false);
        assert_eq!(is_leap_year(1999), false);
        assert_eq!(is_leap_year(2000), true); // century but div 400
    }

    #[test]
    fn test_days_in_month() {
        assert_eq!(Month::January.days_in_month(1999), 31);
        assert_eq!(Month::January.days_in_month(2000), 31);
        assert_eq!(Month::February.days_in_month(1999), 28);
        assert_eq!(Month::February.days_in_month(2000), 29);
    }

    #[test]
    fn test_date_order() {
        assert!(Date::new(1, Month::January, 1900) == Date::new(1, Month::January, 1900));
        assert!(Date::new(1, Month::January, 1900) < Date::new(2, Month::January, 1900));
        assert!(Date::new(2, Month::January, 1900) < Date::new(1, Month::February, 1900));
        assert!(Date::new(2, Month::January, 1900) < Date::new(1, Month::January, 1901));
    }

    #[test]
    fn test_date_range() {
        let mut range = DateRange::new(
            Date::new(1, Month::January, 1900),
            Date::new(4, Month::January, 1900),
        );

        assert_eq!(range.next(), Some(Date::new(1, Month::January, 1900)));
        assert_eq!(range.next(), Some(Date::new(2, Month::January, 1900)));
        assert_eq!(range.next(), Some(Date::new(3, Month::January, 1900)));
        assert_eq!(range.next(), None);
    }

    #[test]
    fn test_number_of_sundays_on_first_of_month() {
        // 2023-10-01 was a sunday

        let mut range = DateRange::new(
            Date::new(1, Month::September, 2023),
            Date::new(1, Month::December, 2023),
        );
        assert_eq!(range.number_of_sundays_on_first_of_month(), 1);

        let mut range = DateRange::new(
            Date::new(1, Month::October, 2023),
            Date::new(1, Month::December, 2023),
        );
        assert_eq!(range.number_of_sundays_on_first_of_month(), 1);

        let mut range = DateRange::new(
            Date::new(1, Month::November, 2023),
            Date::new(1, Month::December, 2023),
        );
        assert_eq!(range.number_of_sundays_on_first_of_month(), 0);
    }
}
