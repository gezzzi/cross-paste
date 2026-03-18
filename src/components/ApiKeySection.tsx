import { useState } from "react";

interface Props {
  apiKey: string;
  onRegenerate: () => Promise<string>;
}

export function ApiKeySection({ apiKey, onRegenerate }: Props) {
  const [visible, setVisible] = useState(false);
  const [confirming, setConfirming] = useState(false);

  const handleRegenerate = async () => {
    if (!confirming) {
      setConfirming(true);
      return;
    }
    await onRegenerate();
    setConfirming(false);
  };

  return (
    <div className="card">
      <div className="card-header">
        <h2>API Key</h2>
      </div>
      <div className="card-body">
        <div className="api-key-display">
          <code className="api-key-value">
            {visible ? apiKey : "\u2022".repeat(32)}
          </code>
          <div className="api-key-actions">
            <button
              className="btn-small"
              onClick={() => setVisible(!visible)}
            >
              {visible ? "Hide" : "Show"}
            </button>
            <button
              className="btn-small"
              onClick={() => navigator.clipboard.writeText(apiKey)}
            >
              Copy
            </button>
            <button
              className={`btn-small ${confirming ? "btn-danger" : ""}`}
              onClick={handleRegenerate}
              onBlur={() => setConfirming(false)}
            >
              {confirming ? "Confirm?" : "Regenerate"}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
