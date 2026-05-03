import { invoke } from "@tauri-apps/api/core";

export async function sendRequest(req: {
  url: string;
  method: string;
  body?: string;
}) {
  return await invoke("send_request", { req });
}