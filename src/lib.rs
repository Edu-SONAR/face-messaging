#![no_std]

#[allow(unused_imports)]
#[macro_use]
extern crate alloc;

use alloc::{string::String, vec::Vec};
use core::time::Duration;
use num_complex::Complex64;
use serde::{Deserialize, Serialize};

const NUM_ELEMENTS: usize = 16;

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct JobId(u32);
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct EventId(u32);
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct BeamId(u32);
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TxDataId(u32);

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum Command {
    Job(Job),
    StateCommand(StateCommand),
    TxData(TxData),
    PowerConfigCommand(PowerConfigCommand),
    ConfigCommand(ConfigCommand),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum Response {
    JobResults(JobResults),
    ConfigResponse(ConfigResponse),
    ParseError(ParseError),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Job {
    pub id: JobId,
    pub duration: Duration,
    pub num_repeats: u32,
    pub events: Vec<Event>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum Event {
    TxEvent(TxEvent),
    RxEvent(RxEvent),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct TxEvent {
    pub start: Duration,
    pub duration: Duration,
    pub tx_data_id: TxDataId,
    pub steering_vec: SteeringVec,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct RxEvent {
    pub start: Duration,
    pub duration: Duration,
    pub steering_vecs: Vec<SteeringVec>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct SteeringVec {
    pub id: BeamId,
    pub coefficients: [Complex64; NUM_ELEMENTS],
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct JobResults {
    pub id: JobId,
    pub rx_data: Vec<RxData>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct RxData {
    pub id: EventId,
    pub beams: Vec<Beam>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Beam {
    pub id: BeamId,
    pub data: Vec<u32>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct StateCommand {}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct TxData {
    pub id: TxDataId,
    pub audio_data: Vec<u32>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct PowerConfigCommand {}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct ConfigCommand {}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct ConfigResponse {}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct ParseError {
    pub msg: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::fmt::Debug;
    use postcard::{from_bytes, to_allocvec};
    use serde::{Deserialize, Serialize};

    fn check_serialization<T: Serialize + for<'a> Deserialize<'a> + PartialEq + Debug>(t: &T) {
        let serialized = to_allocvec(t).unwrap();
        let deserialized = from_bytes::<T>(&serialized).unwrap();
        assert_eq!(t, &deserialized);
    }

    #[test]
    fn serde_job() {
        let command = Command::Job(example_job());
        check_serialization(&command);
    }

    fn example_job() -> Job {
        Job {
            id: JobId(0),
            duration: Duration::from_millis(10),
            num_repeats: 10,
            events: vec![
                Event::TxEvent(TxEvent {
                    start: Duration::ZERO,
                    duration: Duration::from_micros(400),
                    tx_data_id: TxDataId(0),
                    steering_vec: SteeringVec {
                        id: BeamId(0),
                        coefficients: [Complex64::new(1.0, 1.0); 16],
                    },
                }),
                Event::RxEvent(RxEvent {
                    start: Duration::from_micros(500),
                    duration: Duration::from_micros(400),
                    steering_vecs: vec![SteeringVec {
                        id: BeamId(0),
                        coefficients: [Complex64::new(1.0, 1.0); 16],
                    }],
                }),
            ],
        }
    }
}
