import ResponseHeaders from "./ResponseHeaders";
import ResponseBody from "./ResponseBody";

interface Props {
  response: string;
  headers: Record<string, string>;
}

export default function ResponseSection({
  response,
  headers,
}: Props) {
  return (
    <div className="response-section">

      <div className="panel-title">
        Response
      </div>

      <div className="response-content">

        <ResponseHeaders headers={headers} />

        <ResponseBody response={response} />

      </div>

    </div>
  );
}