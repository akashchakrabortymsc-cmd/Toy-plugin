use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::process::Command;

use plugin_core::{Job, JobId, RpcRequest, RpcResponse};

fn main() -> std::io::Result<()> {
    // 1. Bind to a free port — OS picks the number.
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    println!("[host] listening on 127.0.0.1:{port}");

    // 2. Spawn the plugin binary, passing the port as its only argument.
    let plugin_path = std::env::args()
        .nth(1)
        .expect("usage: plugin-host <path-to-plugin-binary>");

    let mut child = Command::new(&plugin_path)
        .arg(port.to_string())
        .spawn()
        .unwrap_or_else(|e| panic!("failed to spawn plugin at {plugin_path}: {e}"));

    // 3. Wait for the plugin to connect back.
    let (stream, _) = listener.accept()?;
    println!("[host] plugin connected");

    let mut writer = stream.try_clone()?;
    let mut reader = BufReader::new(stream);

    // 4. Submit a job.
    let job = Job {
        id: JobId("job-1".to_string()),
        command: "echo Hello from plugin!".to_string(),
    };
    let req = RpcRequest {
        method: "submit".to_string(),
        params: serde_json::to_value(&job).unwrap(),
    };
    send(&mut writer, &req)?;
    let res = recv(&mut reader)?;
    println!("[host] submit response: {res:?}");

    // 5. Ask for its status.
    let req = RpcRequest {
        method: "status".to_string(),
        params: serde_json::to_value(&job.id).unwrap(),
    };
    send(&mut writer, &req)?;
    let res = recv(&mut reader)?;
    println!("[host] status response: {res:?}");

    let _ = child.wait();
    Ok(())
}

fn send(writer: &mut impl Write, req: &RpcRequest) -> std::io::Result<()> {
    let line = serde_json::to_string(req).unwrap();
    writeln!(writer, "{line}")
}

fn recv(reader: &mut impl BufRead) -> std::io::Result<RpcResponse> {
    let mut line = String::new();
    reader.read_line(&mut line)?;
    Ok(serde_json::from_str(&line).unwrap())
}
