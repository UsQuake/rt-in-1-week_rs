use std::fmt;
use std::io::{self, Write};
pub struct IndicateBar 
{
    pub max_progress: u32,
    pub progress: u32
}
impl fmt::Display for IndicateBar
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut progressed_bar = String::from("[");
        for _ in 0..self.progress * 15 / self.max_progress
        {
            progressed_bar.push('\u{1f525}');
        }
        if self.progress < self.max_progress
        {
            progressed_bar.push('\u{1f680}');
        }
       
        for _ in self.progress * 15 / self.max_progress..14
        {
            progressed_bar.push('-');
        }
        progressed_bar.push_str("");
        return write!(f, "{}] [{}/{}]\r", progressed_bar, self.progress, self.max_progress);
    }
}
impl IndicateBar
{
    pub fn new(max: u32) ->Self
    {
        Self
        {
            max_progress: max,
            progress: 0
        }
    }

    pub fn progress(&mut self, add_progress: u32)
    {
        self.progress += add_progress;
        print!("\u{1f3a8} : \x1b[96m{}\x1b[0m\r", self);
        io::stdout().flush().unwrap();
    } 
    pub fn progress_one(&mut self)
    {
        self.progress(1);
    }
}