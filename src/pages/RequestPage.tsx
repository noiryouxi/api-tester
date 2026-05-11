import { useState } from "react";
import RequestSection from "../components/request/RequestSection";
import ResponseSection from "../components/response/ResponseSection";
import { sendRequest, type HttpMethod } from "../services/tauri/http";

export default function RequestPage() {
  const [method, setMethod] = useState<HttpMethod>("POST");
  const [url, setUrl] = useState("");

  const [body, setBody] = useState(`{
  "name": "test"
}`);

  const [headers, setHeaders] = useState([
    { key: "Content-Type", value: "application/json" },
  ]);

  const [response, setResponse] = useState("");
  const [responseHeaders, setResponseHeaders] = useState<any>({});

  const [loading, setLoading] = useState(false);

  const onSend = async () => {
    setLoading(true);

    try {
      const headerObject = headers.reduce((acc, h) => {
        if (h.key.trim()) {
          acc[h.key] = h.value;
        }
        return acc;
      }, {} as Record<string, string>);

      const res = await sendRequest({
        method,
        url,
        body,
        headers: headerObject,
      });

      setResponse(JSON.stringify(res.body, null, 2));
      setResponseHeaders(res.headers || {});
    } catch (e: any) {
      setResponse("❌ " + e.message);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="request-page">

      <RequestSection
        method={method}
        setMethod={setMethod}
        url={url}
        setUrl={setUrl}
        body={body}
        setBody={setBody}
        headers={headers}
        setHeaders={setHeaders}
        onSend={onSend}
        loading={loading}
      />

      <ResponseSection
        response={response}
        headers={responseHeaders}
      />

    </div>
  );
}