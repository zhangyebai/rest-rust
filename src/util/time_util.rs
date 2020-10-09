use chrono::{NaiveTime, NaiveDateTime, Local, NaiveDate};

pub fn time(time: NaiveTime) -> String {
    time_opt(time, "%H:%M:%S")
}

// 格式化时间
// %H for hours of 24
// %M for minutes of 60
// %S for seconds of 60
pub fn time_opt(time: NaiveTime, opt: &str) -> String {
    time.format(opt).to_string()
}

pub fn time_now() -> String {
    time(Local::now().time())
}

pub fn time_now_opt(opt: &str) -> String {
    time_opt(Local::now().time(), opt)
}

pub fn to_time(time: &str) -> Option<NaiveTime>{
    to_time_opt(time, "%H:%M:%S")
}

pub fn to_time_opt(time: &str, opt: &str) -> Option<NaiveTime>{
    match NaiveTime::parse_from_str(time, opt){
        Ok(t) => Some(t),
        _ => None,
    }
}


pub fn date(date: NaiveDate) -> String {
    date_opt(date, "%Y-%m-%d")
}

pub fn date_opt(date: NaiveDate, opt: &str) -> String {
    date.format(opt).to_string()
}

pub fn date_now() -> String{
    date(Local::today().naive_local())
}

pub fn date_now_opt(opt: &str) -> String{
    date_opt(Local::today().naive_local(), opt)
}

pub fn to_date(date: &str) -> Option<NaiveDate>{
    to_date_opt(date, "%Y-%m-%d")
}

pub fn to_date_opt(date: &str, opt: &str) -> Option<NaiveDate>{
    match NaiveDate::parse_from_str(date, opt){
        Ok(d) => Some(d),
        _ => None,
    }
}



pub fn date_time(dt: NaiveDateTime) -> String{
    date_time_opt(dt, "%Y-%m-%d %H:%M:%S")
}

pub fn date_time_opt(dt: NaiveDateTime, opt: &str) -> String{
    dt.format(opt).to_string()
}

pub fn date_time_now() -> String{
    date_time(Local::now().naive_local())
}

pub fn date_time_now_opt(opt: &str) -> String{
    date_time_opt(Local::now().naive_local(), opt)
}

pub fn to_date_time(date_time: &str) -> Option<NaiveDateTime>{
    to_date_time_opt(date_time, "%Y-%m-%d %H:%M:%S")
}

pub fn to_date_time_opt(date_time: &str, opt: &str) -> Option<NaiveDateTime>{
    match NaiveDateTime::parse_from_str(date_time, opt){
        Ok(dt) => Some(dt),
        _ => None,
    }
}

#[test]
pub fn dt_test(){
    println!("time: {}", time_now());
    println!("date: {}", date_now());
    println!("date_time: {}", date_time_now());
}