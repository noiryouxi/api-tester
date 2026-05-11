interface Props {
  response: string;
}

export default function ResponseBody({
  response,
}: Props) {
  return (
    <div className="response-body">

      <div className="panel-title">
        Body
      </div>

      <div className="card">
        <h3>Response</h3>
        <pre className="response">{response}</pre>
      </div>
    </div>
  );
}