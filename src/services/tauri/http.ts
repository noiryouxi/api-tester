import { invoke } from "@tauri-apps/api/core";

/**
 * 지원하는 HTTP 메서드 타입
 * - 문자열 대신 유니온 타입으로 제한해서 오타 방지
 */
export type HttpMethod = "GET" | "POST" | "PUT" | "DELETE";

/**
 * 프론트 → Tauri(Rust)로 전달하는 요청 타입
 * 
 * Rust의 HttpRequest 구조와 맞추는 것이 중요
 * (필드 이름/구조가 다르면 invoke에서 에러 발생)
 */
export interface HttpRequest {
  url: string;
  method: HttpMethod;

  // 선택적 헤더 (ex: Authorization, Content-Type 등)
  headers?: Record<string, string>;

  // 요청 바디 (현재는 string 기반, 필요하면 Uint8Array로 확장 가능)
  body?: string;
}

/**
 * Tauri backend에서 반환하는 응답 타입
 */
export interface HttpResponse {
  status: number;
  body: string;
}

/**
 * Tauri의 send_request 커맨드를 호출하는 함수
 * 
 * 역할:
 * - React → Rust HTTP 엔진 연결
 * - 요청/응답 타입을 안전하게 유지
 * - 에러를 일관된 형태로 변환
 */
export async function sendRequest(req: HttpRequest): Promise<HttpResponse> {
  try {
    // invoke로 Rust command 호출
    // 제네릭으로 응답 타입을 명시해서 타입 안정성 확보
    const res = await invoke<HttpResponse>("send_request", { req });

    return res;
  } catch (err) {
    // Tauri 에러는 string 또는 unknown 형태로 올 수 있음
    console.error("Request failed:", err);

    // UI에서 처리하기 쉽게 Error 객체로 변환
    throw new Error(
      typeof err === "string" ? err : "Unknown error occurred"
    );
  }
}