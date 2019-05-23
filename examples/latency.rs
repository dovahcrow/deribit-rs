#![feature(async_await)]

use deribit::models::TestRequest;
use deribit::DeribitBuilder;
use failure::Fallible;
use futures::compat::Stream01CompatExt;
use futures::{Future, FutureExt, StreamExt, TryFutureExt};
#[cfg(target_os = "linux")]
use scheduler::{set_self_policy, Policy};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::runtime::Builder;
use tokio::timer::Interval;


fn main() -> Fallible<()> {
    let fut = async move {
        let drb = DeribitBuilder::default().build().unwrap();
        let (mut client, _) = drb.connect().await?;

        let mut stats = StatsCollection::new();
        let mut timer = Interval::new_interval(Duration::from_secs(1)).compat();

        while let Some(_) = timer.next().await {
            for _ in 0..100 {
                stats.start("Total");
                let fut = stats
                    .with("Send", client.call(TestRequest::default()))
                    .await?;
                stats.with("Receive", fut).await?;
                stats.end("Total");
            }

            println!("{}", stats);
            // stats.reset();
        }

        Ok(())
    };

    start(fut)

}

#[cfg(target_os = "linux")]
fn start<F>(f: F) -> Fallible<()>
where
    F: Future<Output = Fallible<()>> + Send + 'static,
{
    let fut = f.boxed().compat();
    let mut rt = Builder::new()
        .after_start(|| {
            set_self_policy(Policy::RoundRobin, 99).expect("Cannot set policy");
            // set_self_affinity(CpuSet::new(1)).expect("Cannot set affinity");
        })
        .build()
        .expect("Cannot create runtime");
    rt.block_on(fut)
}


#[cfg(not(target_os = "linux"))]
fn start<F>(f: F) -> Fallible<()>
where
    F: Future<Output = Fallible<()>> + Send + 'static,
{
    let fut = f.boxed().compat();
    let mut rt = Builder::new().build().expect("Cannot create runtime");
    rt.block_on(fut)
}


pub struct StatsCollection {
    stats: HashMap<&'static str, Stats>,
}

impl StatsCollection {
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
        }
    }

    pub fn start(&mut self, name: &'static str) {
        self.stats.entry(name).or_insert_with(Stats::new).start()
    }

    pub fn get(&mut self, name: &'static str) -> &Stats {
        self.stats.entry(name).or_insert_with(Stats::new);
        &self[name]
    }

    pub fn end(&mut self, name: &'static str) {
        self.stats.get_mut(name).expect("Stats haven't start").end()
    }

    pub fn reset(&mut self) {
        for stats in self.stats.values_mut() {
            stats.clear();
        }
    }

    pub async fn with<'a, T: 'a>(
        &'a mut self,
        name: &'static str,
        f: impl Future<Output = T> + 'a,
    ) -> T {
        self.start(name);
        let ret = f.await;
        self.end(name);
        ret
    }
}

impl std::ops::Index<&'static str> for StatsCollection {
    type Output = Stats;
    fn index(&self, idx: &'static str) -> &Stats {
        &self.stats[idx]
    }
}

impl std::fmt::Display for StatsCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut v = vec![];
        for (name, stats) in &self.stats {
            v.push(format!("{}: {}", name, stats));
        }
        write!(f, "{}", v.join(", "))
    }
}

#[derive(PartialEq)]
pub struct Stats {
    pub count: u32,
    pub max: Duration,
    pub min: Duration,
    pub total: Duration,
    pub start_t: Option<Instant>,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            count: 0,
            max: Duration::new(0, 0),
            min: Duration::new(999, 0),
            total: Duration::new(0, 0),
            start_t: None,
        }
    }

    pub fn start(&mut self) {
        self.start_t = Some(Instant::now())
    }

    pub fn end(&mut self) {
        let elapsed: Duration = self.start_t.take().unwrap().elapsed();
        self.count += 1;
        self.total += elapsed;
        self.max = self.max.max(elapsed);
        self.min = self.min.min(elapsed);
    }

    pub async fn with<'a, T: 'a>(&'a mut self, f: impl Future<Output = T> + 'a) -> T {
        self.start();
        let ret = f.await;
        self.end();
        ret
    }

    pub fn clear(&mut self) {
        self.count = 0;
        self.max = Duration::new(0, 0);
        self.min = Duration::new(100, 0);
        self.total = Duration::new(0, 0);
    }
}

impl std::fmt::Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let avg = if self.count == 0 {
            self.total
        } else {
            self.total / self.count
        };
        write!(
            f,
            "[{:?}, {:?}] ~ {:?} ({})",
            self.min, self.max, avg, self.count
        )
    }
}
