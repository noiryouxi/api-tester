export type HttpMethod = "GET" | "POST" | "PUT" | "DELETE";

export interface RequestState {
  method: HttpMethod;
  url: string;
  body?: string;
}

export interface ResponseState {
  status?: number;
  data: string;
}