use rand::Rng;

pub fn random_int_s(length: u32) -> String {
    if length == 0 {
        return "".to_string();
    }

    let mut int_s: String = String::new();
    for _ in 0..length {
        int_s += rand::thread_rng().gen_range(0, 10).to_string().as_str();
    }
    int_s
}

pub fn long_to_ip_v4(lip: u64) -> String {
    String::new()
        +
        (lip >> 24).to_string().as_str()
        +
        "."
        +
        ((lip & 0x00FF_FFFF) >> 16).to_string().as_str()
        +
        "."
        +
        ((lip & 0x0000_FFFF) >> 8).to_string().as_str()
        +
        "."
        +
        (lip & 0x0000_00FF).to_string().as_str()
}

pub fn ip_v4_to_long(ip: &str) -> u64 {
    let tks: Vec<&str> = ip.split('.').collect();
    if tks.len() != 4 {
        return 0u64;
    }

    let p1 = match (*tks.get(0).unwrap()).to_string().parse::<u64>() {
        Ok(p) => p << 24,
        _ => return 0u64,
    };

    let p2 = match (*tks.get(1).unwrap()).to_string().parse::<u64>() {
        Ok(p) => p << 16,
        _ => return 0u64,
    };

    let p3 = match (*tks.get(2).unwrap()).to_string().parse::<u64>() {
        Ok(p) => p << 8,
        _ => return 0u64,
    };

    let p4 = match (*tks.get(3).unwrap()).to_string().parse::<u64>() {
        Ok(p) => p,
        _ => return 0u64,
    };

    p1 + p2 + p3 + p4
}

pub fn blur_phone_mid(phone: &str) -> String{
    match phone {
        p if p.len() == 11 => String::from(&p[0..3]) + "****" + &p[7..],
        _ => String::from(phone),
    }
}

pub fn blur_phone_prefix(phone: &str) -> String{
    match phone {
        p if p.len() == 11 => String::from("****") + &p[4..],
        _ => String::from(phone),
    }
}

pub fn blur_phone_suffix(phone: &str) -> String{
    match phone {
        p if p.len() == 11 => String::from(&p[0..7]) + "****",
        _ => String::from(phone),
    }
}

#[test]
pub fn random_int_s_test() {
    println!("7 - {}, 6 - {}", random_int_s(7), random_int_s(6))
}

#[test]
pub fn long_to_ip_v4_test(){
    let ip = ip_v4_to_long("192.168.0.1");
    println!("ip = {}", ip);

    let ip_str = long_to_ip_v4(3232235521);
    println!("ip_str = {}", ip_str);
    assert_eq!(ip_str, "192.168.0.1")
}

#[test]
pub fn blur_phone_test(){
    let phone = "18612345678";
    let phone_prefix = blur_phone_prefix(phone);
    println!("phone_prefix = {}", phone_prefix);

    let phone_mid = blur_phone_mid(phone);
    println!("phone_mid = {}", phone_mid);

    let phone_suffix = blur_phone_suffix(phone);
    println!("phone_suffix = {}", phone_suffix);
}