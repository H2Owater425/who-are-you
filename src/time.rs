use crate::constant::{DAY_OF_WEEKS, MONTHS, MONTH_LENGTHS, YEAR_LENGTHS};
use std::{error::Error, time::{SystemTime, UNIX_EPOCH}};

pub struct Time {
	pub second: i32,
	pub minute: i32,
	pub hour: i32,
	pub day: i32,
	pub month: i32,
	pub year: i32,
	pub day_of_week: i32,
	pub day_of_year: i32
}

impl Time {
	pub fn now() -> Result<Self, Box<dyn Error>> {
		let mut time: Time = Time {
			second: 0,
			minute: 0,
			hour: 0,
			day: 0,
			month: 0,
			year: 1970,
			day_of_week: 0,
			day_of_year: 0
		};

		let now: i64 = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;
		let mut remainder: i32 = now as i32 % 86400;
		let mut is_leap_year: bool = false;
		let mut new_year: i32;

		time.day_of_year = now as i32 / 86400;
		time.hour = remainder / 3600;
		remainder = remainder % 3600;
		time.minute = remainder / 60;
		time.second = remainder % 60;
		time.day_of_week = (4 + time.day_of_year) % 7;

		while time.day_of_year < 0 || time.day_of_year >= YEAR_LENGTHS[is_leap_year as usize] {
			new_year = time.year + time.day_of_year / 365;

			if time.day_of_year < 0 {
				new_year -= 1;
			}
			
			time.day_of_year -= (new_year - time.year) * 365 + Self::leaps_through_end_of(new_year - 1) - Self::leaps_through_end_of(time.year - 1);
			time.year = new_year;

			is_leap_year = Self::is_leap(time.year)
		}

		time.day = time.day_of_year;

		let month_lengths: [i32; 12] = MONTH_LENGTHS[is_leap_year as usize];

		while time.day >= month_lengths[time.month as usize] {
			time.day -= month_lengths[time.month as usize];
			time.month += 1;
		}

		time.day += 1;

		Ok(time)
	}

	fn is_leap(year: i32) -> bool {
		year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
	}

	fn leaps_through_end_of(year: i32) -> i32 {
		year / 4 - year / 100 + year / 400
	}

	pub fn as_imf_fixdate(&self) -> String {
		format!("{}, {} {} {} {:0>2}:{:0>2}:{:0>2} GMT", DAY_OF_WEEKS[self.day_of_week as usize], self.day, MONTHS[self.month as usize], self.year, self.hour, self.minute, self.second)
	}
}