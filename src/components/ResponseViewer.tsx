export default function ResponseViewer({ response }: any) {
  return (
    <div className="card">
      <h3>Response</h3>
      <pre className="response">{response}</pre>
    </div>
  );
}