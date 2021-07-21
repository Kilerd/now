use chrono::{DateTime, Datelike, Duration, Month, NaiveDate, TimeZone, Timelike, Utc};
use std::ops::{Add, Sub};

pub trait TimeZoneNow {
    type Timezone: TimeZone;
    fn now(&self) -> DateTime<Self::Timezone>;
    fn beginning_of_minute(&self) -> DateTime<Self::Timezone>;
    fn beginning_of_hour(&self) -> DateTime<Self::Timezone>;
    fn beginning_of_day(&self) -> DateTime<Self::Timezone>;
    fn beginning_of_week(&self) -> DateTime<Self::Timezone>;
    fn beginning_of_month(&self) -> DateTime<Self::Timezone>;
    fn beginning_of_quarter(&self) -> DateTime<Self::Timezone>;
    fn beginning_of_year(&self) -> DateTime<Self::Timezone>;

    fn end_of_minute(&self) -> DateTime<Self::Timezone>;
    fn end_of_hour(&self) -> DateTime<Self::Timezone>;
    fn end_of_day(&self) -> DateTime<Self::Timezone>;
    fn end_of_week(&self) -> DateTime<Self::Timezone>;
    fn end_of_month(&self) -> DateTime<Self::Timezone>;
    fn end_of_quarter(&self) -> DateTime<Self::Timezone>;
    fn end_of_year(&self) -> DateTime<Self::Timezone>;
}

pub trait DateTimeNow {
    type Timezone: TimeZone;
    fn beginning_of_minute(&self) -> DateTime<Self::Timezone>;
    fn beginning_of_hour(&self) -> DateTime<Self::Timezone>;
    fn beginning_of_day(&self) -> DateTime<Self::Timezone>;
    fn beginning_of_week(&self) -> DateTime<Self::Timezone>;
    fn beginning_of_month(&self) -> DateTime<Self::Timezone>;
    fn beginning_of_quarter(&self) -> DateTime<Self::Timezone>;
    fn beginning_of_year(&self) -> DateTime<Self::Timezone>;

    fn end_of_minute(&self) -> DateTime<Self::Timezone>;
    fn end_of_hour(&self) -> DateTime<Self::Timezone>;
    fn end_of_day(&self) -> DateTime<Self::Timezone>;
    fn end_of_week(&self) -> DateTime<Self::Timezone>;
    fn end_of_month(&self) -> DateTime<Self::Timezone>;
    fn end_of_quarter(&self) -> DateTime<Self::Timezone>;
    fn end_of_year(&self) -> DateTime<Self::Timezone>;
}

impl<T> TimeZoneNow for T
where
    T: TimeZone,
{
    type Timezone = T;

    fn now(&self) -> DateTime<Self::Timezone> {
        Utc::now().with_timezone(&self)
    }

    fn beginning_of_minute(&self) -> DateTime<Self::Timezone> {
        Utc::now().with_timezone(self).beginning_of_minute()
    }

    fn beginning_of_hour(&self) -> DateTime<Self::Timezone> {
        Utc::now().with_timezone(self).beginning_of_hour()
    }

    fn beginning_of_day(&self) -> DateTime<Self::Timezone> {
        Utc::now().with_timezone(self).beginning_of_day()
    }

    fn beginning_of_week(&self) -> DateTime<Self::Timezone> {
        Utc::now().with_timezone(self).beginning_of_week()
    }

    fn beginning_of_month(&self) -> DateTime<Self::Timezone> {
        Utc::now().with_timezone(self).beginning_of_month()
    }

    fn beginning_of_quarter(&self) -> DateTime<Self::Timezone> {
        Utc::now().with_timezone(self).beginning_of_quarter()
    }

    fn beginning_of_year(&self) -> DateTime<Self::Timezone> {
        Utc::now().with_timezone(self).beginning_of_year()
    }

    fn end_of_minute(&self) -> DateTime<Self::Timezone> {
        Utc::now().with_timezone(self).end_of_minute()
    }

    fn end_of_hour(&self) -> DateTime<Self::Timezone> {
        Utc::now().with_timezone(self).end_of_hour()
    }

    fn end_of_day(&self) -> DateTime<Self::Timezone> {
        Utc::now().with_timezone(self).end_of_day()
    }

    fn end_of_week(&self) -> DateTime<Self::Timezone> {
        Utc::now().with_timezone(self).end_of_week()
    }

    fn end_of_month(&self) -> DateTime<Self::Timezone> {
        Utc::now().with_timezone(self).end_of_month()
    }

    fn end_of_quarter(&self) -> DateTime<Self::Timezone> {
        Utc::now().with_timezone(self).end_of_quarter()
    }

    fn end_of_year(&self) -> DateTime<Self::Timezone> {
        Utc::now().with_timezone(self).end_of_year()
    }
}

impl<T: TimeZone> DateTimeNow for DateTime<T> {
    type Timezone = T;

    fn beginning_of_minute(&self) -> DateTime<Self::Timezone> {
        let local_date_time = self.naive_local();
        let time5 = NaiveDate::from_ymd(
            local_date_time.year(),
            local_date_time.month(),
            local_date_time.day(),
        )
        .and_hms(local_date_time.hour(), local_date_time.minute(), 0);
        self.timezone().from_local_datetime(&time5).unwrap()
    }

    fn beginning_of_hour(&self) -> DateTime<Self::Timezone> {
        let local_date_time = self.naive_local();
        let time5 = NaiveDate::from_ymd(
            local_date_time.year(),
            local_date_time.month(),
            local_date_time.day(),
        )
        .and_hms(local_date_time.hour(), 0, 0);
        self.timezone().from_local_datetime(&time5).unwrap()
    }

    fn beginning_of_day(&self) -> DateTime<Self::Timezone> {
        let local_date_time = self.naive_local();
        let time5 = NaiveDate::from_ymd(
            local_date_time.year(),
            local_date_time.month(),
            local_date_time.day(),
        )
        .and_hms(0, 0, 0);
        self.timezone().from_local_datetime(&time5).unwrap()
    }

    fn beginning_of_week(&self) -> DateTime<Self::Timezone> {
        let prec_day = self.weekday().number_from_monday() - 1;
        let time: DateTime<T> = self.clone().sub(Duration::days(prec_day as i64));
        let succ_local_date_time = time.naive_local();
        let time5 = NaiveDate::from_ymd(
            succ_local_date_time.year(),
            succ_local_date_time.month(),
            succ_local_date_time.day(),
        )
        .and_hms(0, 0, 0);
        self.timezone().from_local_datetime(&time5).unwrap()
    }

    fn beginning_of_month(&self) -> DateTime<Self::Timezone> {
        let local_date_time = self.naive_local();
        let time5 = NaiveDate::from_ymd(local_date_time.year(), local_date_time.month(), 1)
            .and_hms(0, 0, 0);
        self.timezone().from_local_datetime(&time5).unwrap()
    }

    fn beginning_of_quarter(&self) -> DateTime<Self::Timezone> {
        let local_date_time = self.naive_local();
        let month = match local_date_time.month() {
            1..=3 => 1u32,
            4..=6 => 4u32,
            7..=9 => 7u32,
            _ => 10u32,
        };
        let time5 = NaiveDate::from_ymd(local_date_time.year(), month, 1).and_hms(0, 0, 0);
        self.timezone().from_local_datetime(&time5).unwrap()
    }

    fn beginning_of_year(&self) -> DateTime<Self::Timezone> {
        let local_date_time = self.naive_local();
        let time5 = NaiveDate::from_ymd(local_date_time.year(), 1, 1).and_hms(0, 0, 0);
        self.timezone().from_local_datetime(&time5).unwrap()
    }

    fn end_of_minute(&self) -> DateTime<Self::Timezone> {
        let local_date_time = self.naive_local();
        let time5 = NaiveDate::from_ymd(
            local_date_time.year(),
            local_date_time.month(),
            local_date_time.day(),
        )
        .and_hms_nano(
            local_date_time.hour(),
            local_date_time.minute(),
            59,
            999999999,
        );
        self.timezone().from_local_datetime(&time5).unwrap()
    }

    fn end_of_hour(&self) -> DateTime<Self::Timezone> {
        let local_date_time = self.naive_local();
        let time5 = NaiveDate::from_ymd(
            local_date_time.year(),
            local_date_time.month(),
            local_date_time.day(),
        )
        .and_hms_nano(local_date_time.hour(), 59, 59, 999999999);
        self.timezone().from_local_datetime(&time5).unwrap()
    }

    fn end_of_day(&self) -> DateTime<Self::Timezone> {
        let local_date_time = self.naive_local();
        let time5 = NaiveDate::from_ymd(
            local_date_time.year(),
            local_date_time.month(),
            local_date_time.day(),
        )
        .and_hms_nano(23, 59, 59, 999999999);
        self.timezone().from_local_datetime(&time5).unwrap()
    }

    fn end_of_week(&self) -> DateTime<Self::Timezone> {
        let succ_day = 7 - self.weekday().number_from_monday();
        let time: DateTime<T> = self.clone().add(Duration::days(succ_day as i64));
        let succ_local_date_time = time.naive_local();
        let time5 = NaiveDate::from_ymd(
            succ_local_date_time.year(),
            succ_local_date_time.month(),
            succ_local_date_time.day(),
        )
        .and_hms_nano(23, 59, 59, 999999999);
        self.timezone().from_local_datetime(&time5).unwrap()
    }

    fn end_of_month(&self) -> DateTime<Self::Timezone> {
        let local_date_time = self.naive_local();
        let (year, month) = if local_date_time.month() == Month::December.number_from_month() {
            (
                local_date_time.year() + 1,
                Month::January.number_from_month(),
            )
        } else {
            (local_date_time.year(), local_date_time.month() + 1)
        };

        let time5 = NaiveDate::from_ymd(year, month, 1).and_hms(0, 0, 0);
        self.timezone()
            .from_local_datetime(&time5)
            .unwrap()
            .sub(Duration::nanoseconds(1))
    }

    fn end_of_quarter(&self) -> DateTime<Self::Timezone> {
        let local_date_time = self.naive_local();
        let (year, month) = match local_date_time.month() {
            1..=3 => (local_date_time.year(), 4u32),
            4..=6 => (local_date_time.year(), 7u32),
            7..=9 => (local_date_time.year(), 10u32),
            _ => (local_date_time.year() + 1, 1u32),
        };
        let time5 = NaiveDate::from_ymd(year, month, 1).and_hms(0, 0, 0);
        self.timezone()
            .from_local_datetime(&time5)
            .unwrap()
            .sub(Duration::nanoseconds(1))
    }

    fn end_of_year(&self) -> DateTime<Self::Timezone> {
        let local_date_time = self.naive_local();
        let time5 =
            NaiveDate::from_ymd(local_date_time.year(), 12, 31).and_hms_nano(23, 59, 59, 999999999);
        self.timezone().from_local_datetime(&time5).unwrap()
    }
}

#[cfg(test)]
mod test {
    use chrono::Timelike;
    use crate::TimeZoneNow;

    #[test]
    fn test_end_of_day() {
        use chrono::FixedOffset;

        use crate::TimeZoneNow;
        let offset = FixedOffset::east(60 * 60 * 8);
        let x = offset.end_of_day();
        assert_eq!(23, x.hour());
        assert_eq!(59, x.minute());
        assert_eq!(59, x.second());
    }

    #[test]
    fn test_all() {
        use chrono::FixedOffset;

        use crate::TimeZoneNow;
        let offset = FixedOffset::east(60 * 60 * 8);

        dbg!(offset.now());
        dbg!(offset.beginning_of_minute());
        dbg!(offset.beginning_of_hour());
        dbg!(offset.beginning_of_day());
        dbg!(offset.beginning_of_week());
        dbg!(offset.beginning_of_month());
        dbg!(offset.beginning_of_quarter());
        dbg!(offset.beginning_of_year());

        dbg!(offset.end_of_minute());
        dbg!(offset.end_of_hour());
        dbg!(offset.end_of_day());
        dbg!(offset.end_of_week());
        dbg!(offset.end_of_month());
        dbg!(offset.end_of_quarter());
        dbg!(offset.end_of_year());
    }
}
