
interface HeaderItem {
  key: string;
  value: string;
}

interface Props {
  headers: HeaderItem[];
  setHeaders: (headers: HeaderItem[]) => void;
}

export default function RequestHeaders({
  headers,
  setHeaders,
}: Props) {
  const addHeader = () => {
    setHeaders([
      ...headers,
      { key: "", value: "" },
    ]);
  };

  const removeHeader = (index: number) => {
    setHeaders(
      headers.filter((_, i) => i !== index)
    );
  };

  const updateHeader = (
    index: number,
    field: "key" | "value",
    value: string
  ) => {
    const copy = [...headers];
    copy[index][field] = value;
    setHeaders(copy);
  };

  return (
    <div className="headers-panel">

      <div className="panel-title">
        Headers
      </div>

      <div className="headers-list">

        {headers.map((header, index) => (
          <div
            key={index}
            className="header-row"
          >
            <input
              type="text"
              placeholder="Key"
              value={header.key}
              onChange={(e) =>
                updateHeader(
                  index,
                  "key",
                  e.target.value
                )
              }
            />

            <input
              type="text"
              placeholder="Value"
              value={header.value}
              onChange={(e) =>
                updateHeader(
                  index,
                  "value",
                  e.target.value
                )
              }
            />

            <button
              className="delete-btn"
              onClick={() =>
                removeHeader(index)
              }
            >
              ×
            </button>
          </div>
        ))}
      </div>

      <button
        className="add-header-btn"
        onClick={addHeader}
      >
        + Add Header
      </button>
    </div>
  );
}