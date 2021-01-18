// https://tools.ietf.org/html/rfc8011#section-5.3.7
#[derive(Copy, Clone, Debug)]
pub enum JobState {
    Pending = 0x03,
    PendingHeld = 0x04,
    Processing = 0x05,
    ProcessingStopped = 0x06,
    Canceled = 0x07,
    Aborted = 0x08,
    Completed = 0x09,
}

impl From<JobState> for String {
    fn from(s: JobState) -> Self {
        match s {
            JobState::Pending => String::from("pending"),
            JobState::PendingHeld => String::from("pending-held"),
            JobState::Processing => String::from("processing"),
            JobState::ProcessingStopped => String::from("processing-stopped"),
            JobState::Canceled => String::from("canceled"),
            JobState::Aborted => String::from("aborted"),
            JobState::Completed => String::from("completed"),
        }
    }
}

// https://tools.ietf.org/html/rfc8011#section-5.3.8
#[derive(Copy, Clone, Debug)]
pub enum JobStateReason {
    None,
    // TODO: Add all values
    QueuedInDevice,
}

impl From<JobStateReason> for String {
    fn from(r: JobStateReason) -> Self {
        match r {
            JobStateReason::None => String::from("none"),
            JobStateReason::QueuedInDevice => String::from("queued-in-device"),
        }
    }
}
