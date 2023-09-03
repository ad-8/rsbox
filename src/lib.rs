use chrono::prelude::*;
use std::path::{Path};

pub fn prefix_with_datetime(filename: &Path, dt: &DateTime<Local>) -> String {
    let filename = filename.to_str().unwrap();
    let dt_fmt = dt.format("%Y%m%d_%H%M%S").to_string();
    format!("{}_{}", dt_fmt, filename)
}

/// Adds date and time in front of a filename, e.g. _foo.txt_ becomes _20230121_111728_foo.txt_.
pub fn prefix_with_datetime_now(filename: &Path) -> String {
    let now: DateTime<Local> = Local::now();
    prefix_with_datetime(filename, &now)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn test_prefix_with_datetime() {
        let dt = Local
            .from_local_datetime(&NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2020, 12, 24).unwrap(),
                NaiveTime::from_hms_opt(20, 15, 00).unwrap(),
            ))
            .unwrap();

        let result = prefix_with_datetime(&PathBuf::from("foo.rs"), &dt);
        assert_eq!(result, "20201224_201500_foo.rs");
    }
}
