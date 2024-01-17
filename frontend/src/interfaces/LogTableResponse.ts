import {LogEntry} from "@/interfaces/LogEntry";

export interface LogTableResponse {
  logs: LogEntry[];
  total_items: number;
}
