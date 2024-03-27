use once_cell::sync::Lazy;

use crate::hookimpl::SpeedHackers;

pub static  mut G_EXE_NAME:Lazy<String>=Lazy::new(|| String::from(""));
pub static mut RTL_QUERY_PERFORMANCE_COUNTER_HACKER:Lazy<SpeedHackers<u64>>=Lazy::new(|| SpeedHackers::new(1.0,0,0));
pub static mut GET_TICK_COUNT:Lazy<SpeedHackers<i32>>=Lazy::new(|| SpeedHackers::new(1.0,0,0));
pub static mut GET_TICK_COUNT64:Lazy<SpeedHackers<u64>>=Lazy::new(|| SpeedHackers::new(1.0,0,0));
pub static mut TIME_GET_TIME:Lazy<SpeedHackers<i32>>=Lazy::new(|| SpeedHackers::new(1.0,0,0));