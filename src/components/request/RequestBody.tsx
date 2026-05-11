interface Props {
  body: string;
  setBody: (body: string) => void;
  disabled: boolean;
}

export default function RequestBody({
  body,
  setBody,
  disabled,
}: Props) {
  return (
    <div className="body-panel">

      <div className="panel-title">
        Body
      </div>

      <div className="card">
      <h3>Request Body</h3>
      <textarea
        className="textarea"
        value={body}
        onChange={(e) => setBody(e.target.value)}
        disabled={disabled}
      />
    </div>
    </div>
  );
}