use serde::{Deserialize, Serialize};

/// Unique identifier for a submitted job.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct JobId(pub String);

impl std::fmt::Display for JobId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Current state of a job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Cancelled,
    Failed(String),
}

/// A unit of work the engine wants a plugin to run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: JobId,
    pub command: String,
}

/// A JSON-RPC-style request sent over TCP — one line of JSON.
#[derive(Debug, Serialize, Deserialize)]
pub struct RpcRequest {
    pub method: String,
    pub params: serde_json::Value,
}

/// The response to an RpcRequest — also one line of JSON.
#[derive(Debug, Serialize, Deserialize)]
pub struct RpcResponse {
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn job_id_displays() {
        let id = JobId("job-1".to_string());
        assert_eq!(id.to_string(), "job-1");
    }

    #[test]
    fn job_id_round_trips_json() {
        let id = JobId("job-42".to_string());
        let json = serde_json::to_string(&id).unwrap();
        let back: JobId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, back);
    }
}
