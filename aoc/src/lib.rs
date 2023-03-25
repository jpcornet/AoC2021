use std::time::{Duration, Instant};
use std::io::BufRead;
use std::fmt::Display;

// ExRunner runs an exercise and keeps status
pub struct ExRunner<'a> {
    name: String,
    start: Instant,
    answ: [Option<Box<dyn Display + 'a>>; 2],
    parsetime: Option<Duration>,
    runtime: [Option<Duration>; 2],
    totaltime: Option<Duration>,
}

impl<'a> ExRunner<'a> {
    pub fn run<T: BufRead>(name: String, f: fn(&T, &ExRunner), input: &T) -> ExRunner {
        let start = Instant::now();
        let mut r = ExRunner { name, start, ..Default::default() };
        f(input, &r);
        r.totaltime = Some(start.elapsed());
        r
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    fn part_x<T>(&mut self, part: usize, answ: T)
        where T: Display + 'a
    {
        let elapsed = self.start.elapsed();
        match self.answ[part] {
            None => self.answ[part].insert(Box::new(answ)),
            Some(_) => panic!("Cannot give part{} twice", part + 1),
        };
        let i = match self.runtime[0] {
            None => 0,
            _ => 1,
        };
        self.runtime[i] = Some(elapsed);
    }

    pub fn part1<T>(&mut self, answ: T)
        where T: Display + 'a
    {
        self.part_x(0, answ);
    }

    pub fn part2<T>(&mut self, answ: T)
        where T: Display + 'a
    {
        self.part_x(1, answ);
    }

    pub fn parse_done(&mut self) {
        match self.parsetime {
            None => self.parsetime.insert(self.start.elapsed()),
            Some(_) => panic!("Parsing done twice??"),
        };
    }
}

impl<'a> Default for ExRunner<'a> {
    fn default() -> ExRunner<'a> {
        ExRunner {
            name: "".to_string(),
            start: Instant::now(),
            answ: [None, None],
            parsetime: None,
            runtime: [None; 2],
            totaltime: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_exrunner() {
        let mut run = ExRunner{
            name: "foo".to_string(),
            answ: [Some(Box::new(1)), None],
            ..Default::default()
        };
        assert_eq!(run.name, "foo".to_string());
        assert_eq!(run.answ[0].take().unwrap().to_string(), "1".to_string());
    }
}
