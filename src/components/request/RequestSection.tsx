import RequestControls from "./RequestControls";
import RequestHeaders from "./RequestHeaders";
import RequestBody from "./RequestBody";

export default function RequestSection(props: any) {
  return (
    <div className="request-section">

      <RequestControls
        method={props.method}
        setMethod={props.setMethod}
        url={props.url}
        setUrl={props.setUrl}
        onSend={props.onSend}
        loading={props.loading}
      />

      <div className="request-content">

        <RequestHeaders
          headers={props.headers}
          setHeaders={props.setHeaders}
        />

        <RequestBody
          body={props.body}
          setBody={props.setBody}
          disabled={
            props.method === "GET" ||
            props.method === "DELETE"
          }
        />

      </div>
    </div>
  );
}