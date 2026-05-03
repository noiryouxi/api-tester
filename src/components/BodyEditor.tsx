export default function BodyEditor({
  body,
  setBody,
  disabled,
}: any) {
  return (
    <div className="card">
      <h3>Request Body</h3>
      <textarea
        className="textarea"
        value={body}
        onChange={(e) => setBody(e.target.value)}
        disabled={disabled}
      />
    </div>
  );
}