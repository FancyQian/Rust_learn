pub trait HitronSummary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }

    fn happy(&self) -> String {
        format!("{}: happy day!!!! ", self.summarize())
    }
}

pub struct Suzhou {
    pub boss: String,
    pub location: String,
    pub pplnumber: u32,
    pub sw_kind: String,
}

impl HitronSummary for Suzhou {
    // fn summarize(&self) -> String {
    //     format!("BOSS is {}, Location at {}, total ppl is {})", self.boss, self.location, self.pplnumber)
    // }

    fn happy(&self) -> String {
        format!("@{}: happy Suzhou", self.summarize())
    }
}

pub struct HC {
    pub boss: String,
    pub location: String,
    pub depart_kind: String,
    pub depart_pplnum: u32,
}

impl HitronSummary for HC {
    fn summarize(&self) -> String {
        format!("BOSS is {}, Location at {}, depart {} total ppl number is {})", self.boss, self.location, self.depart_kind, self.depart_pplnum)
    }

    // fn happy(&self) -> String {
    //     format!("@{}: happy hc", self.summarize())
    // }
}

pub fn notify<T: HitronSummary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}