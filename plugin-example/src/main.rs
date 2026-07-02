use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::process::Command;

use plugin_core::{Job, JobId, JobStatus, RpcRequest, RpcResponse};

fn main() -> std::io::Result<()> {
    let port: u16 = std::env::args()
        .nth(1)
        .expect("usage: plugin-example <port>")
        .parse()
        .expect("port must be a number");

    // Connect back to the host.
    let stream = TcpStream::connect(("127.0.0.1", port))?;
    let mut writer = stream.try_clone()?;
    let mut reader = BufReader::new(stream);

    // In-memory job state.
    let mut jobs: HashMap<JobId, (JobStatus, String)> = HashMap::new();

    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            break; // host closed connection
        }

        let request: RpcRequest = serde_json::from_str(&line).unwrap();
        let method = request.method.clone();

        let response = match method.as_str() {
            "submit" => handle_submit(request, &mut jobs),
            "status" => handle_status(request, &jobs),
            other => RpcResponse {
                result: None,
                error: Some(format!("unknown method: {other}")),
            },
        };

        let out = serde_json::to_string(&response).unwrap();
        writeln!(writer, "{out}")?;
    }

    Ok(())
}

fn handle_submit(
    request: RpcRequest,
    jobs: &mut HashMap<JobId, (JobStatus, String)>,
) -> RpcResponse {
    let job: Job = serde_json::from_value(request.params).unwrap();

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/C").arg(&job.command).output()
    } else {
        Command::new("sh").arg("-c").arg(&job.command).output()
    };

    let (status, stdout) = match output {
        Ok(out) if out.status.success() => (
            JobStatus::Completed,
            String::from_utf8_lossy(&out.stdout).to_string(),
        ),
        Ok(out) => (
            JobStatus::Failed(String::from_utf8_lossy(&out.stderr).to_string()),
            String::new(),
        ),
        Err(e) => (JobStatus::Failed(e.to_string()), String::new()),
    };

    jobs.insert(job.id.clone(), (status, stdout));

    RpcResponse {
        result: Some(serde_json::to_value(&job.id).unwrap()),
        error: None,
    }
}

fn handle_status(request: RpcRequest, jobs: &HashMap<JobId, (JobStatus, String)>) -> RpcResponse {
    let id: JobId = serde_json::from_value(request.params).unwrap();
    let (status, stdout) = jobs
        .get(&id)
        .cloned()
        .unwrap_or((JobStatus::Pending, String::new()));

    RpcResponse {
        result: Some(serde_json::json!({
            "status": status,
            "stdout": stdout
        })),
        error: None,
    }
}
