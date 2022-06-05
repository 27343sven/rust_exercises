#[allow(unused)]

const YEAR_SECONDS: f64 = ((60 * 60 * 24) as f64) * 365.25;

#[derive(Debug)]
pub struct Duration {
    years: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {
            years: s as f64 / YEAR_SECONDS,
        }
    }
}

macro_rules! impl_planet {
    ($($t:ty : $n:literal,)+ ) => {
        $(impl Planet for $t {
            fn years_during(d: &Duration) -> f64 {
                d.years / $n
            }
        })*
    };
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl_planet!(
    Mercury: 0.2408467,
    Venus: 0.61519726,
    Earth: 1.0,
    Mars: 1.8808158,
    Jupiter: 11.862615,
    Saturn: 29.447498,
    Uranus: 84.016846,
    Neptune:  164.79132,
);

fn assert_in_delta(expected: f64, actual: f64) {
    let diff: f64 = (expected - actual).abs();
    let delta: f64 = 0.01;

    if diff > delta {
        panic!(
            "Your result of {} should be within {} of the expected result {}",
            actual, delta, expected
        )
    }
}

#[test]
fn earth_age() {
    let duration = Duration::from(1_000_000_000);
    assert_in_delta(31.69, Earth::years_during(&duration));
}

#[test]
fn mercury_age() {
    let duration = Duration::from(2_134_835_688);
    assert_in_delta(280.88, Mercury::years_during(&duration));
}

#[test]
fn venus_age() {
    let duration = Duration::from(189_839_836);
    assert_in_delta(9.78, Venus::years_during(&duration));
}
