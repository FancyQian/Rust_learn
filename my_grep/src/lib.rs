
use std::fs;
use std::error::Error;
use std::env;

pub struct GrepConfig {
    pub g_string: String,
    pub g_file: String,
}

impl GrepConfig {
    pub fn new(mut args: env::Args) -> Result<GrepConfig, &'static str> {
        args.next();

        let g_file = match args.next() {
            Some(file) => file,
            None => return Err("Miss file name!"),
        };

        let g_string = match args.next() {
            Some(str) => str,
            None => return Err("Miss search string!"),
        };

        Ok(GrepConfig { g_string, g_file })
    }

    pub fn run(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let s = fs::read_to_string(&self.g_file)?;

        let ret = GrepConfig::search(&s, &self.g_string)?;
        Ok(ret)
    }

    fn search(full_ctx: & str, query_string: &str) -> Result<Vec<String>, &'static str> {
        let v: Vec<String> = full_ctx.lines().filter(|line| line.contains(query_string)).collect();
 
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