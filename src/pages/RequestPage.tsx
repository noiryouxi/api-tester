import { useState } from "react";
import RequestBar from "../components/RequestBar";
import BodyEditor from "../components/BodyEditor";
import ResponseViewer from "../components/ResponseViewer";
import { sendRequest, type HttpMethod } from "../services/tauri/http";

export default function RequestPage() {
  // HTTP 메서드 상태 (GET, POST, PUT, DELETE 등)
  const [method, setMethod] = useState<HttpMethod>("POST");

  // 요청 URL 상태
  const [url, setUrl] = useState("");

  // 요청 바디(JSON 문자열 형태)
  const [body, setBody] = useState(`{
  "name": "test"
}`);

  // 서버 응답 결과를 저장하는 상태
  const [response, setResponse] = useState("");

  // 요청 진행 중 여부 (로딩 UI 제어용)
  const [loading, setLoading] = useState(false);

  // API 요청 실행 함수
  const onSend = async () => {
    // 요청 시작 → 로딩 ON
    setLoading(true);

    // 이전 응답 초기화
    setResponse("");

    try {
      // Tauri(Rust backend)로 HTTP 요청 전달
      const res = await sendRequest({
        method,
        url,
        body,
      });

      // 응답 결과를 보기 좋게 JSON 문자열로 변환해서 저장
      setResponse(JSON.stringify(res, null, 2));
    } catch (e: any) {
      // 에러 발생 시 메시지 출력
      setResponse("❌ " + e.message);
    } finally {
      // 요청 완료 → 로딩 OFF
      setLoading(false);
    }
  };

  return (
    <div className="container">
      {/* 페이지 제목 */}
      <h1 className="title">API Tester</h1>

      {/* 요청 설정 바 (method, url, send 버튼) */}
      <RequestBar
        method={method}
        setMethod={setMethod}
        url={url}
        setUrl={setUrl}
        onSend={onSend}
        loading={loading}
      />

      {/* 요청 body 입력 + 응답 출력 영역 */}
      <div className="grid">

        {/* JSON body 에디터 */}
        <BodyEditor
          body={body}
          setBody={setBody}
          // GET / DELETE는 body 사용 안 하니까 비활성화
          disabled={method === "GET" || method === "DELETE"}
        />

        {/* 서버 응답 출력 영역 */}
        <ResponseViewer response={response} />
      </div>
    </div>
  );
}