#[allow(dead_code)]

pub mod hitron_sz {
    pub mod rd_bu {
        pub fn new(name: &str, num: u32, department: &str) -> super::PplInfo {
            super::PplInfo::new(name, num, department )
        }
    }

    pub enum HitronBU {
        RdSubBuName(String),
        SvtSubBuName(String),
    }

    pub struct PplInfo {
        pub username: String,
        pub usernumber: u32,
        pub department: HitronBU,
    }

    impl PplInfo {
        pub fn new(name: &str, num: u32, depart: &str) -> PplInfo {
            PplInfo {
                username: String::from(name),
                usernumber: num,
                department: HitronBU::RdSubBuName(String::from(depart))
            }
        }
    }
}