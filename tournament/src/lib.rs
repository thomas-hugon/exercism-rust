use crate::matches::Status::{Draw, Lose, Win};
use crate::matches::{Stat, Team};

mod matches {
    use crate::matches::Status::{Draw, Lose, Win};
    use std::collections::HashMap;
    use std::fmt;
    use std::fmt::{Display, Formatter};

    #[derive(PartialEq, Hash, Eq, Ord, PartialOrd)]
    pub struct Team<'a>(&'a str);
    impl<'a> Team<'a> {
        pub fn new(team: &'a str) -> Self {
            Team(team)
        }
    }
    impl<'a> Display for Team<'a> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(Default)]
    pub struct Stat {
        pub mp: u8,
        pub w: u8,
        pub d: u8,
        pub l: u8,
        pub p: u8
    }

    pub enum Status {
        Win,
        Lose,
        Draw,
    }

    pub struct Stats<'a>(HashMap<Team<'a>, Stat>);
    impl<'a> Stats<'a> {
        pub fn new() -> Self {
            Stats(HashMap::new())
        }
        fn push(&mut self, status: Status, team: Team<'a>) {
            let Stat { mp, w, d, l, p } = self.0.entry(team).or_insert_with(Stat::default);
            match status {
                Win => *w += 1,
                Lose => *l += 1,
                Draw => *d += 1
            }
            *mp = *w + *l +*d;
            *p = *w*3 + *d;
        }
        pub fn score(&mut self, status: Status, team1: Team<'a>, team2: Team<'a>) {
            match status {
                Win => {
                    self.push(Win, team1);
                    self.push(Lose, team2);
                }
                Lose => {
                    self.push(Lose, team1);
                    self.push(Win, team2);
                }
                Draw => {
                    self.push(Draw, team1);
                    self.push(Draw, team2);
                }
            }
        }

        pub fn stats(&self) -> Vec<(&Team, &Stat)> {
            self.0
                .keys()
                .flat_map(|k| self.0.get_key_value(k))
                .try_fold(Vec::new(), |mut acc, v| {
                    acc.push(v);
                    Some(acc)
                })
                .unwrap_or_default()
        }
    }
}

pub fn tally(match_results: &str) -> String {
    let mut stats = matches::Stats::new();

    match_results
        .split('\n')
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let s: Vec<&str> = line.split(';').collect();
            match s[2] {
                "win" => stats.score(Win, Team::new(s[0]), Team::new(s[1])),
                "loss" => stats.score(Lose, Team::new(s[0]), Team::new(s[1])),
                "draw" => stats.score(Draw, Team::new(s[0]), Team::new(s[1])),
                _ => panic!(),
            }
        });

    let mut stats = stats.stats();
    stats.sort_by(|(t1, s1), (t2, s2)|s2.p.cmp(&s1.p).then_with(||t1.cmp(&t2)));

    std::iter::once(
        String::from("Team                           | MP |  W |  D |  L |  P")
    )
    .chain(stats.iter().map(
        |(team, Stat { mp, w, d, l, p })|
        format!( "{:30} | {:2} | {:2} | {:2} | {:2} | {:2}", team.to_string(), mp, w, d, l, p )
    ))
    .collect::<Vec<String>>()
    .join("\n")
}
