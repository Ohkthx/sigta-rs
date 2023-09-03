use tatk::indicators::{DEMA, EMA, SMA};
#[cfg(feature = "test-data")]
use tatk::test_data::TEST_DATA;
use tatk::traits::Line;

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a SMA using 252 data points with a period of 10.
fn create_sma() {
    let sma = SMA::new(10, TEST_DATA).unwrap();
    assert_eq!(sma.value(), 109.112);
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a SMA from 252 data points and a period of 10, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_sma() {
    let mut sma = SMA::new(10, TEST_DATA).unwrap();
    assert_eq!(sma.next(107.0), 108.81199999999998);
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a EMA using 252 data points with a period of 10.
fn create_ema() {
    let ema = EMA::new(10, TEST_DATA).unwrap();
    assert_eq!(ema.value(), 108.97521174143839)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a EMA from 252 data points and a period of 10, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_ema() {
    let mut ema = EMA::new(10, TEST_DATA).unwrap();
    assert_eq!(ema.next(107.000000), 108.61608233390413)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a DEMA using 252 data points with a period of 10.
fn create_dema() {
    let dema = DEMA::new(14, TEST_DATA).unwrap();
    assert_eq!(dema.value(), 109.46762588466589)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a DEMA from 252 data points and a period of 10, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_dema() {
    let mut dema = DEMA::new(14, TEST_DATA).unwrap();
    assert_eq!(dema.next(107.000000), 108.91612556961066)
}