import { useState } from "react";

interface Props {
  serverUrl: string;
  apiKey: string;
}

export function ShortcutGuide({ serverUrl, apiKey }: Props) {
  const [expanded, setExpanded] = useState(false);

  return (
    <div className="card">
      <div
        className="card-header clickable"
        onClick={() => setExpanded(!expanded)}
      >
        <h2>iPhone Shortcuts Setup Guide</h2>
        <span className="expand-icon">{expanded ? "\u25B2" : "\u25BC"}</span>
      </div>
      {expanded && (
        <div className="card-body guide-body">
          <section>
            <h3>Shortcut 1: Send to PC (iPhone → Windows)</h3>
            <ol>
              <li>Open the <strong>Shortcuts</strong> app</li>
              <li>Tap <strong>+</strong> to create a new Shortcut</li>
              <li>
                Add action: <strong>Get Clipboard</strong>
              </li>
              <li>
                Add action: <strong>Get Contents of URL</strong>
                <ul>
                  <li>
                    URL:{" "}
                    <code
                      className="copyable"
                      onClick={() =>
                        navigator.clipboard.writeText(
                          `${serverUrl}/api/clipboard`
                        )
                      }
                    >
                      {serverUrl}/api/clipboard
                    </code>
                  </li>
                  <li>Method: <strong>POST</strong></li>
                  <li>
                    Headers:
                    <ul>
                      <li>
                        <code>Authorization</code>:{" "}
                        <code
                          className="copyable"
                          onClick={() =>
                            navigator.clipboard.writeText(`Bearer ${apiKey}`)
                          }
                        >
                          Bearer {apiKey.substring(0, 8)}...
                        </code>
                      </li>
                      <li>
                        <code>Content-Type</code>:{" "}
                        <code>application/json</code>
                      </li>
                    </ul>
                  </li>
                  <li>
                    Request Body (JSON):
                    <pre>
{`{
  "content_type": "text",
  "data": "<Clipboard>"
}`}
                    </pre>
                    <small>Set "data" value to the Clipboard variable</small>
                  </li>
                </ul>
              </li>
              <li>(Optional) Add action: <strong>Show Notification</strong> - "Sent to PC!"</li>
            </ol>
          </section>

          <section>
            <h3>Shortcut 2: Get from PC (Windows → iPhone)</h3>
            <ol>
              <li>Open the <strong>Shortcuts</strong> app</li>
              <li>Tap <strong>+</strong> to create a new Shortcut</li>
              <li>
                Add action: <strong>Get Contents of URL</strong>
                <ul>
                  <li>
                    URL:{" "}
                    <code
                      className="copyable"
                      onClick={() =>
                        navigator.clipboard.writeText(
                          `${serverUrl}/api/clipboard`
                        )
                      }
                    >
                      {serverUrl}/api/clipboard
                    </code>
                  </li>
                  <li>Method: <strong>GET</strong></li>
                  <li>
                    Headers:
                    <ul>
                      <li>
                        <code>Authorization</code>:{" "}
                        <code
                          className="copyable"
                          onClick={() =>
                            navigator.clipboard.writeText(`Bearer ${apiKey}`)
                          }
                        >
                          Bearer {apiKey.substring(0, 8)}...
                        </code>
                      </li>
                    </ul>
                  </li>
                </ul>
              </li>
              <li>Add action: <strong>Get Dictionary Value</strong> - key "data" from result</li>
              <li>Add action: <strong>Get Dictionary Value</strong> - key "data" from previous</li>
              <li>Add action: <strong>Copy to Clipboard</strong></li>
              <li>(Optional) Add action: <strong>Show Notification</strong></li>
            </ol>
          </section>

          <section className="guide-note">
            <strong>Note:</strong> Both devices must be on the same Wi-Fi network.
            Click on <code className="copyable">code</code> values to copy them.
          </section>
        </div>
      )}
    </div>
  );
}
