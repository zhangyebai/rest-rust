use chrono::Local;

use std::sync::{Arc, Mutex};

use std::thread;
use lazy_static;
use rand::Rng;

struct SnowFlake {
    lock: Mutex<u64>,
    data_center_id: i64,
    machine_id: i64,
    last_time_stamp: i64,
    sequence: i64,
}

// 起始的时间戳 2020-05-03 00:05:03
static START_TIME_STAMP: i64 = 1588435503000i64;

// 序列号占用的位数
static SEQUENCE_BITS: i64 = 12i64;

// 机器标识占用的位数
static MACHINE_BITS: i64 = 5i64;

// 数据中心占用的位数
static DATA_CENTER_BITS: i64 = 5;

// 数据中心最大占位数
//static MAX_DATA_CENTER_NUM: i64 = -1i64 ^ (-1i64 << DATA_CENTER_BITS);

// 机器码最大占位数
//static MAX_MACHINE_NUM: i64 = -1i64 ^ (-1i64 << MACHINE_BITS);

// 序列号最大占位数
static MAX_SEQUENCE: i64 = -1i64 ^ (-1i64 << SEQUENCE_BITS);

// 机器码移位
static MACHINE_LEFT: i64 = SEQUENCE_BITS;

// 数据中心码移位
static DATA_CENTER_LEFT: i64 = SEQUENCE_BITS + MACHINE_BITS;

// 时间戳移位
static TIME_STAMP_LEFT: i64 = DATA_CENTER_LEFT + DATA_CENTER_BITS;

impl SnowFlake {
    pub fn new(data_center_id: i64, machine_id: i64) -> SnowFlake {
        SnowFlake {
            lock: Mutex::new(0),
            data_center_id,
            machine_id,
            last_time_stamp: 0,
            sequence: 0,
        }
    }

    pub fn next_id(&mut self) -> i64 {
        let mut l = self.lock.lock().unwrap();
        *l += 1;
        let mut now_time_stamp = Local::now().timestamp_millis() as i64;
        if now_time_stamp == self.last_time_stamp {
            self.sequence = (self.sequence + 1) & MAX_SEQUENCE;
            if self.sequence == 0 {
                now_time_stamp += 1;
            }
        } else {
            self.sequence = 0i64;
        }
        self.last_time_stamp = now_time_stamp;

        ((now_time_stamp - START_TIME_STAMP) << TIME_STAMP_LEFT) | (self.data_center_id << DATA_CENTER_LEFT) | (self.machine_id << MACHINE_LEFT) | (self.sequence)
    }
}

lazy_static! {
    static ref SNOW_FLAKE_WORKER_0: Arc<Mutex<SnowFlake>> = Arc::new(Mutex::new(SnowFlake::new(1, 1)));
    static ref SNOW_FLAKE_WORKER_1: Arc<Mutex<SnowFlake>> = Arc::new(Mutex::new(SnowFlake::new(1, 2)));
    static ref SNOW_FLAKE_WORKER_2: Arc<Mutex<SnowFlake>> = Arc::new(Mutex::new(SnowFlake::new(1, 3)));
    static ref SNOW_FLAKE_WORKER_3: Arc<Mutex<SnowFlake>> = Arc::new(Mutex::new(SnowFlake::new(1, 4)));

}

//static mut SNOW_FLAKE_WORKER_0: SnowFlake = SnowFlake::new(1, 1);

pub fn id() -> String {
    //SNOW_FLAKE_WORKER_0.next_id().to_string()
    let mut worker = (*SNOW_FLAKE_WORKER_0).lock().unwrap();
    worker.next_id().to_string()
}

pub fn balanced_id() -> String{
    match rand::thread_rng().gen_range(0, 4){
        0 => (*SNOW_FLAKE_WORKER_0).lock().unwrap().next_id().to_string(),
        1 => (*SNOW_FLAKE_WORKER_1).lock().unwrap().next_id().to_string(),
        2 => (*SNOW_FLAKE_WORKER_2).lock().unwrap().next_id().to_string(),
        3 => (*SNOW_FLAKE_WORKER_3).lock().unwrap().next_id().to_string(),
        _ => (*SNOW_FLAKE_WORKER_0).lock().unwrap().next_id().to_string(),
    }
}

#[test]
pub fn test_snow_flake_id() {
    for i in 0..10 {
        thread::spawn(move || {
            for idx in 0..20 {
                println!("thread-{}:{}:{}", i, idx, balanced_id());
            }
        }).join();
    }
}