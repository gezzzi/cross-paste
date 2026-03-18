export interface Settings {
  port: number;
  api_key: string;
  auto_start: boolean;
  bind_address: string;
}

export interface ServerStatus {
  running: boolean;
  port: number;
  local_ip: string;
}
