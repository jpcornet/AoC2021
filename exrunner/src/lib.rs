use std::time::{Duration, Instant};
use std::io::BufRead;
use std::fmt::Display;

// ExRunner runs an exercise and keeps status
pub struct ExRunner<'a> {
    name: String,
    start: Instant,
    answ: [Option<Box<dyn Display + 'a>>; 2],
    label: [String; 2],
    parsetime: Option<Duration>,
    runtime: [Option<Duration>; 2],
    totaltime: Option<Duration>,
}

impl<'a> ExRunner<'a> {
    pub fn run<T: BufRead>(name: String, f: fn(T, &mut ExRunner), input: T) -> ExRunner<'a> {
        let start = Instant::now();
        let mut r = ExRunner { name, start, ..Default::default() };
        f(input, &mut r);
        r.totaltime = Some(start.elapsed());
        r
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    fn part_x<T>(&mut self, part: usize, answ: T, label: Option<&str>)
        where T: Display + 'a
    {
        let elapsed = self.start.elapsed();
        match self.answ[part] {
            None => self.answ[part].insert(Box::new(answ)),
            Some(_) => panic!("Cannot give part{} twice", part + 1),
        };
        self.label[part] = label.unwrap_or(&format!("part{}", part + 1)).to_string();
        let i = match self.runtime[0] {
            None => 0,
            _ => 1,
        };
        self.runtime[i] = Some(elapsed);
    }

    pub fn part1<T>(&mut self, answ: T, label: Option<&str>)
        where T: Display + 'a
    {
        self.part_x(0, answ, label);
    }

    pub fn part2<T>(&mut self, answ: T, label: Option<&str>)
        where T: Display + 'a
    {
        self.part_x(1, answ, label);
    }

    pub fn parse_done(&mut self) {
        match self.parsetime {
            None => self.parsetime.insert(self.start.elapsed()),
            Some(_) => panic!("Parsing done twice??"),
        };
    }

    pub fn answ(&self) -> Vec<Option<String>> {
        self.answ.iter().map(|b| b.as_ref().map(|x| x.to_string())).collect()
    }

    pub fn parsetime(&self) -> Option<Duration> {
        self.parsetime
    }

    pub fn time1(&self) -> Option<Duration> {
        self.runtime[0].map(|d| d - self.parsetime.unwrap_or(Duration::from_secs(0)))
    }

    pub fn time2(&self) -> Option<Duration> {
        self.runtime[1].map(|d| d - self.runtime[0].unwrap_or(Duration::from_secs(0)))
    }

    pub fn cleanuptime(&self) -> Option<Duration> {
        self.totaltime.map(|d| d - self.runtime[1].unwrap_or(
                                                self.runtime[0].unwrap_or(
                                                self.parsetime.unwrap_or(
                                                Duration::from_secs(0)))))
    }

    pub fn totaltime(&self) -> Option<Duration> {
        self.totaltime
    }

    pub fn print_raw(&self) {
        if self.runtime[0].is_none() {
            println!("{} did not produce any answers", self.name);
        } else {
            println!("{}:", self.name);
            let answers = self.answ();
            for i in 0..=1 {
                if let Some(a) = &answers[i] {
                    println!("{}: {}", self.label[i], a);
                }
            }
            if let Some(pt) = self.parsetime {
                println!("Parsing took: {:?}", pt)
            }
            let ordinals = vec!["first", "second"];
            let runtimes = vec![self.time1(), self.time2()];
            for i in 0..=1 {
                if let Some(rt) = runtimes[i] {
                    println!("Calculating {} answer took: {:?}", ordinals[i], rt);
                }
            }
        }
        if let Some(ct) = self.cleanuptime() {
            println!("Cleanup took: {:?}", ct);
        }
        if let Some(tt) = self.totaltime() {
            println!("Total exercise time: {:?}", tt);
        }
    }
}

impl<'a> Default for ExRunner<'a> {
    fn default() -> ExRunner<'a> {
        ExRunner {
            name: "".to_string(),
            start: Instant::now(),
            answ: [None, None],
            label: ["".to_string(), "".to_string()],
            parsetime: None,
            runtime: [None; 2],
            totaltime: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;
    use std::thread;

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

    #[test]
    fn just_part1() {
        let input = BufReader::new("foo".as_bytes());
        let run = ExRunner::run("just_part1".to_string(), |_i, r| r.part1(3, None), input);
        assert_eq!(run.answ(), vec![Some("3".to_string()), None]);
        assert_eq!(*run.name(), "just_part1".to_string());
        assert_eq!(run.label[0], "part1".to_string());
        assert!(run.time1().is_some());
        assert!(run.time2().is_none());
    }

    fn do_two_parts(i: impl BufRead, r: &mut ExRunner) {
        let part1 = i.lines().map(|l| l.unwrap()).collect::<Vec<String>>().join(" ");
        r.part1(part1, None);
        thread::sleep(Duration::from_millis(1));
        r.part2(3.5, Some("Floating point result:"));
    }

    #[test]
    fn two_parts() {
        let input = BufReader::new("foo\nbar\ntest\n".as_bytes());
        let run = ExRunner::run("two_parts".to_string(), do_two_parts, input);
        assert_eq!(run.answ(), vec![Some("foo bar test".to_string()), Some("3.5".to_string())]);
        assert!(run.time1() < Some(Duration::from_millis(1)));
        assert!(run.time2() > Some(Duration::from_millis(1)));
        assert_eq!(run.label[1], "Floating point result:".to_string());
    }

    fn do_only_part2(_i: impl BufRead, r: &mut ExRunner) {
        r.parse_done();
        thread::sleep(Duration::from_millis(1));
        r.part2("static slice here", None);
    }

    #[test]
    fn just_part_two() {
        let input = BufReader::new("nothing".as_bytes());
        let run = ExRunner::run("just_part_two".to_string(), do_only_part2, input);
        assert_eq!(run.answ(), vec![None, Some("static slice here".to_string())]);
        assert!(run.time1() > Some(Duration::from_millis(1)));
        assert!(run.parsetime() < Some(Duration::from_millis(1)));
        assert_eq!(run.time2(), None);
        assert!(run.cleanuptime() < Some(Duration::from_millis(1)));
    }

    fn do_double_part1(_i: impl BufRead, r: &mut ExRunner) {
        r.part1(1, None);
        r.part1(2, None); // this will panic
    }

    #[test]
    #[should_panic(expected = "Cannot give part1 twice")]
    fn double_part1() {
        let input = BufReader::new("nothing".as_bytes());
        let _run = ExRunner::run("double_part1".to_string(), do_double_part1, input);
    }

}
