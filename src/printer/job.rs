use crate::printer::jobstate::{JobState, JobStateReason};

#[derive(Clone, Debug)]
pub struct Job {
    pub id: u32,
    pub uri: String,
    pub state: JobState,
    pub state_reasons: Vec<JobStateReason>,
    data: Vec<u8>,
}

impl Job {
    pub fn new(id: u32, uri: String, data: &[u8]) -> Self {
        Self {
            id,
            uri,
            state: JobState::Pending,
            state_reasons: vec![JobStateReason::None],
            data: data.to_vec(),
        }
    }
}
