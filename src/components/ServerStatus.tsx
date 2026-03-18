import type { ServerStatus as ServerStatusType } from "../types";

interface Props {
  status: ServerStatusType | null;
}

export function ServerStatus({ status }: Props) {
  if (!status) return null;

  const url = `http://${status.local_ip}:${status.port}`;

  return (
    <div className="card">
      <div className="card-header">
        <span
          className={`status-dot ${status.running ? "running" : "stopped"}`}
        />
        <h2>Server Status</h2>
      </div>
      <div className="card-body">
        <div className="info-row">
          <span className="label">Status</span>
          <span className={status.running ? "text-green" : "text-red"}>
            {status.running ? "Running" : "Stopped"}
          </span>
        </div>
        <div className="info-row">
          <span className="label">URL</span>
          <span className="url-value">
            <code>{url}</code>
            <button
              className="btn-small"
              onClick={() => navigator.clipboard.writeText(url)}
              title="Copy URL"
            >
              Copy
            </button>
          </span>
        </div>
      </div>
    </div>
  );
}
