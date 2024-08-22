use serde::de;

const CLEAR: &str = "\x1B[2J\x1B[1;1H"; // clear the console

fn progress<Iter>(iter: Iter, f: fn(Iter::Item))
where Iter: Iterator {
    let mut i = 1;
    for n in iter {
        println!("{}{}", CLEAR, "*".repeat(i));
        i += 1;
        f(n);
    }
}

fn delay(_n: &i32) {
    std::thread::sleep(std::time::Duration::from_secs(1));
}

pub(crate) fn run() {
    let v = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30];
    let brkts = ('|', '|');
    // for n in Progress::new(v.iter()) {
    //     delay(n);
    // }
    // or 
    v
    .iter()
    .progress()
    .with_bound()
    .with_delims(brkts)
    .for_each(|n| delay(n));

    for n in (1..).progress() {
        delay(&n);
    }
    // let mut h = std::collections::HashSet::new();
    // h.insert(0);

}

struct Progress<Iter> {
    iter: Iter,
    i: usize,
    bound: Option<usize>,
    delims: (char, char),
}

impl<Iter> Progress<Iter> {
    fn new(iter: Iter) -> Self {
        Self { iter, i: 1, bound: None, delims: ('[', ']') }
    }
}

impl<Iter> Progress<Iter>
where Iter: ExactSizeIterator {
    fn with_bound(mut self) -> Self {
        self.bound = Some(self.iter.len());
        self
    }

    fn with_delims(mut self, delims: (char, char)) -> Self {
        self.delims = delims;
        self
    }
}

impl<Iter> Iterator for Progress<Iter>
where Iter: Iterator
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let percent = self.i * 100 / self.bound.unwrap_or(1);
        print!("{}", CLEAR);
        match self.bound {
            Some(bound) => {
                println!("{}% {} {}{} {}",
                percent,
                self.delims.0,
                "*".repeat(self.i),
                " ".repeat(bound - self.i),
                self.delims.1
            );
            },
            _ => {
                println!("{}", "*".repeat(self.i));
            }
        }
        self.i += 1;
        self.iter.next()
    }
}

trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self>;
}

impl<Iter> ProgressIteratorExt for Iter
where Iter: Iterator
{
    fn progress(self) -> Progress<Self> {
        Progress::new(self)
    }
}