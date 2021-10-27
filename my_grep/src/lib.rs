
use std::fs;
use std::error::Error;

pub struct GrepConfig {
    pub g_string: String,
    pub g_file: String,
}

impl GrepConfig {
    pub fn new(args: &[String]) -> Result<GrepConfig, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let g_string = args[1].clone();
        let g_file = args[2].clone();

        Ok(GrepConfig { g_string, g_file })
    }

    pub fn run(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let s = fs::read_to_string(&self.g_file)?;

        let ret = GrepConfig::search(&s, &self.g_string)?;
        Ok(ret)
    }

    fn search(full_ctx: & str, query_string: &str) -> Result<Vec<String>, &'static str> {
        let mut v = Vec::new();
        for line in full_ctx.lines() {
            if line.contains(query_string)
            {
                v.push(String::from(line));
            }
        }

        if v.len() == 0 {
            Err("Match 0 result!!!")
        } else {
            Ok(v)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn once_search_test()
    {
        let query_string = "I am your father";
        let search_string = "father";

        assert_eq!(vec!["I am your father".to_string()], GrepConfig::search(query_string, search_string).expect("Error"));
    }
}