//! Birthday Paradox Simulation, by Prince Muel info@princemuel.dev
//! Explore the surprising probabilities of the "Birthday Paradox".
//! More info at https://en.wikipedia.org/wiki/Birthday_problem
//! This code is available at https://github.com/princemuel/thrills
//! Tags: short, math, simulation

use std::io::{self, Write};

const MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];
const MAX_BIRTHDAYS: u8 = 100;
const NUM_SIMULATIONS: u32 = 100_000;
const DAYS_IN_YEAR: u16 = 365;

fn main() -> io::Result<()> {
    println!(
        "Birthday Paradox, by Prince Muel info@princemuel.dev

The Birthday Paradox shows us that in a group of N people,
the odds that two of them have matching birthdays is surprisingly large.
This program does a Monte Carlo simulation (repeated random simulations)
to explore this concept.

(It's not actually a paradox, it's just a surprising result.)"
    );

    let num_birthdays = read_num_birthdays()?;
    println!();

    let storage = BirthdayStorage::new(num_birthdays);
    let birthdays = storage.as_slice();
    println!("Here are {} birthdays:", birthdays.len());
    for (i, birthday) in birthdays.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{}", birthday);
    }
    print!("\n\n");

    print!("In this simulation, ");
    match find_duplicate(birthdays) {
        Some(birthday) => println!("multiple people have a birthday on {}", birthday),
        None => println!("there are no matching birthdays."),
    }
    println!();

    let matches = run_simulations(num_birthdays);
    let probability = (matches as f64 / NUM_SIMULATIONS as f64) * 100.0;

    println!(
        "Out of {} simulations of {} people,
there was a matching birthday in that group {} times.
This means that {} people have a {:.2}% chance
of having a matching birthday in their group.
That's probably more than you would think!
",
        NUM_SIMULATIONS, num_birthdays, matches, num_birthdays, probability
    );

    Ok(())
}

fn read_num_birthdays() -> io::Result<u8> {
    loop {
        println!("How many birthdays shall I generate? (Max {})", MAX_BIRTHDAYS);
        print!("> ");
        io::stdout().flush()?;

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;

        if let Ok(response) = buffer.trim().parse()
            && response > 0
            && response <= MAX_BIRTHDAYS
        {
            return Ok(response);
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct Birthday(u16);
impl Birthday {
    #[inline]
    const fn new(day: u16) -> Self {
        debug_assert!(day < DAYS_IN_YEAR);
        Self(day)
    }

    #[inline]
    #[deprecated]
    #[allow(unused)]
    fn randomm() -> Self { Self::new(fastrand::u16(0..DAYS_IN_YEAR)) }

    #[inline]
    fn random() -> Self {
        use rand::prelude::IndexedRandom;
        let mut rng = rand::rng();
        let days: [usize; DAYS_IN_YEAR as usize] = core::array::from_fn(|i| i);

        Self(days.choose(&mut rng).copied().unwrap_or_default() as u16)
    }

    fn month_day(&self) -> (usize, u8) {
        let days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        let mut remaining = self.day_of_year();
        for (month, &days) in days_in_month.iter().enumerate() {
            if remaining < days {
                return (month, (remaining + 1) as u8);
            }

            remaining -= days;
        }

        unreachable!()
    }

    const fn day_of_year(&self) -> u16 { self.0 }
}

impl core::fmt::Display for Birthday {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let (month, day) = self.month_day();
        write!(f, "{} {}", MONTHS[month], day)
    }
}

enum BirthdayStorage {
    Stack([Birthday; MAX_BIRTHDAYS as usize], u8), // array + actual length
    Heap(Vec<Birthday>),
}
impl BirthdayStorage {
    fn new(count: u8) -> Self {
        if count <= MAX_BIRTHDAYS {
            let mut arr = [Birthday::new(0); MAX_BIRTHDAYS as usize];
            for item in arr.iter_mut().take(count as usize) {
                *item = Birthday::random();
            }
            Self::Stack(arr, count)
        } else {
            // Fallback to heap (though we never hit this when MAX=100)
            Self::Heap((0..count).map(|_| Birthday::random()).collect())
        }
    }

    #[inline]
    fn as_slice(&self) -> &[Birthday] {
        match self {
            Self::Stack(arr, len) => &arr[..*len as usize],
            Self::Heap(vec) => vec.as_slice(),
        }
    }
}

fn run_simulations(num_birthdays: u8) -> u32 {
    println!(
        "Generating {n} random birthdays {s} times...",
        n = num_birthdays,
        s = NUM_SIMULATIONS
    );

    println!("Press Enter to begin...");
    let _ = io::stdin().read_line(&mut String::new());

    let progress_interval = NUM_SIMULATIONS / 100;

    let mut matches = 0u32;
    for n in 0..NUM_SIMULATIONS {
        if n % progress_interval == 0 {
            println!("{n} simulations run...");
        }

        let storage = BirthdayStorage::new(num_birthdays);
        // Early return in has_duplicate means we stop checking as soon as we find a
        // match
        if has_duplicate(storage.as_slice()) {
            matches += 1;
        }
    }

    println!("{} simulations run.", NUM_SIMULATIONS);
    matches
}

fn has_duplicate(birthdays: &[Birthday]) -> bool { find_duplicate(birthdays).is_some() }

fn find_duplicate(birthdays: &[Birthday]) -> Option<Birthday> {
    let mut seen = [0u64; 6];

    for &birthday in birthdays {
        let day = birthday.day_of_year() as usize;
        let word = day / 64;
        let bit = day % 64;
        let mask = 1u64 << bit;

        if seen[word] & mask != 0 {
            return Some(birthday);
        }

        seen[word] |= mask;
    }

    None
}

mod fastrand {
    use core::cell::Cell;
    use core::num::Wrapping;
    use std::time::{SystemTime, UNIX_EPOCH};

    thread_local! {
        static RNG: Cell<Wrapping<u64>> = Cell::new(Wrapping(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64
        ));
    }

    #[inline]
    pub fn u16(range: std::ops::Range<u16>) -> u16 {
        let len = range.end - range.start;
        range.start + (u64_to_u16(next()) % len)
    }

    #[inline]
    fn next() -> u64 {
        RNG.with(|rng| {
            let mut n = rng.get();
            // xorshift64*
            n ^= n >> 12;
            n ^= n << 25;
            n ^= n >> 27;
            rng.set(n);
            (n * Wrapping(0x2545_f491_4f6c_dd1d)).0
        })
    }

    #[inline]
    fn u64_to_u16(x: u64) -> u16 { (x >> 48) as u16 }
}
