use serde::{Serialize, Deserialize};
use std::time::Duration;
use log::info;
use rbatis_core::db::PoolOptions;
use rbatis::rbatis::Rbatis;
use lazy_static;


lazy_static! {
  // Rbatis是线程、协程安全的，运行时的方法是Send+Sync，内部使用DashMap等等并发安全的map实现，无需担心线程竞争
  pub static ref REGION_RB: Rbatis=Rbatis::new();
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MysqlConfig {
    pub host: String,
    pub port: u32,
    pub user_name: String,
    pub password: String,
    pub db: String,
    pub max_size: u32,
    // 连接池的上限
    pub connect_timeout: Duration,
    // 连接超时时间
    pub min_size: u32,
    // 连接池的下限
    pub max_lifetime: Option<Duration>,
    // 所有连接的最大生命周期
    pub idle_timeout: Option<Duration>, // 空闲连接的生命周期
}



#[derive(Clone)]
//#[derive(Serialize, Deserialize, Debug)]
pub struct MultiMySqlPool {
    //pub region_pool: Pool<MySqlConnection>,
    //pub region_pool: Rbatis,
}

pub fn find_db_config(active: &str) -> MysqlConfig {
    match active {
        "dev" | "test" => MysqlConfig {
            host: "106.75.245.46".to_string(),
            port: 3306,
            user_name: "root".to_string(),
            password: "1292511ys5".to_string(),
            db: "region".to_string(),
            max_size: 10,
            connect_timeout: Duration::from_secs(5),
            min_size: 4,
            max_lifetime: Some(Duration::from_secs(3)),
            idle_timeout: Some(Duration::from_secs(3)),
        },
        "master" => MysqlConfig {
            host: "106.75.245.46".to_string(),
            port: 3306,
            user_name: "root".to_string(),
            password: "1292511ys5".to_string(),
            db: "region".to_string(),
            max_size: 10,
            connect_timeout: Duration::from_secs(5),
            min_size: 4,
            max_lifetime: Some(Duration::from_secs(3)),
            idle_timeout: Some(Duration::from_secs(3)),
        },
        _ => panic!("illegal profile active value")
    }
}

fn build_mysql_url(config: &MysqlConfig) -> String {
    let mut url = format!("mysql://{}:{}@{}:{}/{}",
                          config.user_name, config.password, config.host, config.port, config.db);
    url
}

pub async fn db_pool(config: &MysqlConfig) -> MultiMySqlPool {
    let mut opt =PoolOptions::new();
    opt.min_size = config.min_size;
    opt.max_size = config.max_size;
    opt.connect_timeout = config.connect_timeout;
    //let mut pool = Rbatis::new();
    (*REGION_RB).link_opt(build_mysql_url(config).as_str(), &opt).await.unwrap();
    MultiMySqlPool {
        //region_pool: pool,
    }
    /*
    let mut conn = format!("mysql://{}:{}@{}:{}", config.user_name, config.password, config.host, config.port);
    if let Some(d) = &config.db{
        conn += format!("/{}", d).as_str();
    }
    info!("db url = {}", conn);
    let pool = sqlx::MySqlPool::builder()
        .max_size(config.max_size)
        .min_size(config.min_size)
        .connect_timeout(config.connect_timeout)
        .max_lifetime(config.max_lifetime)
        .idle_timeout(config.idle_timeout)
        .build(conn.as_str())
        .await
        .unwrap();
    MultiMySqlPool {
        region_pool: pool,
    }*/
}