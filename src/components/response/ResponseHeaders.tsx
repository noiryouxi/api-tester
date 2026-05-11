interface Props {
  headers: Record<string, string>;
}

export default function ResponseHeaders({
  headers,
}: Props) {
  return (
    <div className="response-headers">

      <div className="panel-title">
        Headers
      </div>

      <pre className="response-pre">
        {JSON.stringify(headers, null, 2)}
      </pre>
    </div>
  );
}