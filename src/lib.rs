use chrono::prelude::*;

pub fn prefix_with_datetime(filename: &str, dt: &DateTime<Local>) -> String {
    let dt_fmt = dt.format("%Y%m%d_%H%M%S").to_string();
    format!("{}_{}", dt_fmt, filename)
}

pub fn prefix_with_datetime_now(filename: &str) -> String {
    let now: DateTime<Local> = Local::now();
    prefix_with_datetime(filename, &now)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_with_datetime() {
        let dt = Local.from_local_datetime(&NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2020, 12, 24).unwrap(),
            NaiveTime::from_hms_opt(20, 15, 00).unwrap())).unwrap();

        let result = prefix_with_datetime("foo.rs", &dt);
        assert_eq!(result, "20201224_201500_foo.rs");
    }
}
