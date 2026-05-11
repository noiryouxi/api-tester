
import type { HttpMethod } from "../../services/tauri/http";

interface Props {
  method: HttpMethod;
  setMethod: (method: HttpMethod) => void;
  url: string;
  setUrl: (url: string) => void;
  onSend: () => void;
  loading: boolean;
}

const methods: HttpMethod[] = [
  "GET",
  "POST",
  "PUT",
  "PATCH",
  "DELETE",
  "HEAD",
  "OPTIONS",
  "CONNECT",
  "TRACE",
];

export default function RequestControls({
  method,
  setMethod,
  url,
  setUrl,
  onSend,
  loading,
}: Props) {
  return (
    <div className="request-toolbar">

      <select
        value={method}
        onChange={(e) =>
          setMethod(e.target.value as HttpMethod)
        }
        className="method-select"
      >
        {methods.map((m) => (
          <option key={m} value={m}>
            {m}
          </option>
        ))}
      </select>

      <input
        type="text"
        placeholder="Enter request URL..."
        value={url}
        onChange={(e) => setUrl(e.target.value)}
        className="url-input"
      />

      <button
        onClick={onSend}
        disabled={loading}
        className="send-button"
      >
        {loading ? "Sending..." : "Send"}
      </button>
    </div>
  );
}