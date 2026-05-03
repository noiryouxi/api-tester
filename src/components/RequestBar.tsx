export default function RequestBar({
  method,
  setMethod,
  url,
  setUrl,
  onSend,
  loading,
}: any) {
  return (
    <div className="topBar">
      <select value={method} onChange={(e) => setMethod(e.target.value)}>
        <option>GET</option>
        <option>POST</option>
        <option>PUT</option>
        <option>DELETE</option>
      </select>

      <input
        className="urlInput"
        value={url}
        onChange={(e) => setUrl(e.target.value)}
        placeholder="https://api.example.com"
      />

      <button onClick={onSend} disabled={loading}>
        {loading ? "Sending..." : "Send"}
      </button>
    </div>
  );
}