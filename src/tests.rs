use super::ReDateTime;
use super::repeat_every::RepeatEvery;
use chrono::{Duration, Utc, Datelike, TimeZone};
use chrono::naive::MAX_DATE;

#[test]
fn init() {
    let now = Utc::now();
    let dt = ReDateTime::at(now.clone());

    assert_eq!(dt.at, now);
    assert_eq!(dt.repeat, RepeatEvery::zero());
    assert_eq!(dt.until, max_datetime!());
}

#[test]
fn simple_repeat() {
    let now = Utc::now();
    let wkday = now.weekday();

    let dt = ReDateTime::repeat_until(now, RepeatEvery::weeks(1), now + Duration::weeks(13));

    let mut c = 0;
    for n in dt.iter() {
        assert_eq!(n.weekday(), wkday, "wanted {:?}, got {:?}", wkday, n.weekday());
        c += 1;
    }
    assert_eq!(c, 14);
}

#[test]
fn month_repeat() {
    let now = Utc::now();
    let day = now.day();
    let month = now.month0();

    let dt = ReDateTime::repeat_until(now, RepeatEvery::months(1), now + Duration::days(366));

    let mut c = 0;
    for n in dt.iter() {
        assert_eq!(n.day(), day);
        assert_eq!(n.month0(), (month + c) % 12);
        c += 1;
    }
    assert_eq!(c, 13);
}

#[test]
fn year_repeat() {
    let now = Utc::now();
    let day = now.day();
    let month = now.month();
    let year = now.year();

    let dt = ReDateTime::repeat_until(now, RepeatEvery::years(1), now + Duration::days(1000));

    let mut c = 0;
    for n in dt.iter() {
        assert_eq!(n.day(), day);
        assert_eq!(n.month(), month);
        assert_eq!(n.year(), year + c);
        c += 1;
    }
    assert_eq!(c, 3);
}

#[test]
fn done_before() {
    let dt1 = ReDateTime::repeat(Utc::now(), RepeatEvery::weeks(1));
    let dt2 = ReDateTime::repeat_until(Utc::now(), RepeatEvery::weeks(1),
                                       Utc::now() + Duration::weeks(3));
    let dt3 = ReDateTime::repeat_until(Utc::now(), RepeatEvery::weeks(1),
                                       Utc::now() + Duration::weeks(6));

    let dt5 = Utc::now() + Duration::weeks(5);

    assert!(!dt1.done_before(&dt5));
    assert!(dt2.done_before(&dt5));
    assert!(!dt3.done_before(&dt5));
}

#[test]
fn between() {
    let dt1 = ReDateTime::repeat(Utc::now(), RepeatEvery::weeks(1));
    let dt11 = ReDateTime::repeat_until(Utc::now(), RepeatEvery::weeks(1),
                                        Utc::now() + Duration::weeks(2));

    let dt2 = vec![Utc::now() - Duration::days(1), Utc::now() + Duration::days(1)];
    let dt3 = vec![Utc::now() + Duration::days(1), Utc::now() + Duration::days(3)];
    let dt4 = vec![Utc::now() + Duration::days(20), Utc::now() + Duration::days(22)];

    assert!(dt1.between(&dt2[0], &dt2[1]));
    assert!(!dt1.between(&dt3[0], &dt3[1]));
    assert!(dt1.between(&dt4[0], &dt4[1]));
    assert!(dt11.between(&dt2[0], &dt2[1]));
    assert!(!dt11.between(&dt3[0], &dt3[1]));
    assert!(!dt11.between(&dt4[0], &dt4[1]));
}

#[test]
fn first_after() {
    let now = Utc::now();
    let dt1 = ReDateTime::repeat(now.clone(), RepeatEvery::weeks(1));
    let dt2 = now + Duration::days(15);
    let dt3 = now - Duration::days(15);

    assert_eq!(dt1.first_after(&dt2).unwrap(), now + Duration::weeks(3));
    assert_eq!(dt1.first_after(&dt3).unwrap(), now);
}
