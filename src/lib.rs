use chrono::{DateTime, Datelike, FixedOffset, NaiveDate, NaiveDateTime, TimeZone, Timelike, Utc};

pub trait Now {
    type Timezone: TimeZone;
    fn end_of_today(&self) -> DateTime<Self::Timezone>;
    #[inline]
    fn end_of_today_inline(&self) -> DateTime<Self::Timezone>;
}

impl<T> Now for T
    where
        T: TimeZone,
{
    type Timezone = T;

    fn end_of_today(&self) -> DateTime<Self::Timezone> {
        let time2 = Utc::now().with_timezone(self);
        let time4 = time2.naive_local();
        let time5 = NaiveDate::from_ymd(time4.year(), time4.month(), time4.day()).and_hms(23, 59, 59);
        self.from_local_datetime(&time5).unwrap()
    }
    fn end_of_today_inline(&self) -> DateTime<Self::Timezone> {
        let time2 = Utc::now().with_timezone(self);
        let time4 = time2.naive_local();
        let time5 = NaiveDate::from_ymd(time4.year(), time4.month(), time4.day()).and_hms(23, 59, 59);
        self.from_local_datetime(&time5).unwrap()
    }
}

#[cfg(test)]
mod test {
    use chrono::Timelike;

    #[test]
    fn test() {
        use chrono::FixedOffset;
        use chrono::Utc;

        use crate::Now;
        let offset = FixedOffset::east(60 * 60 * 8);
        let x = offset.end_of_today();
        assert_eq!(23, x.hour());
        println!("{:?}", x);
        println!("{:?}", Utc.end_of_today().with_timezone(&offset));
    }
}
