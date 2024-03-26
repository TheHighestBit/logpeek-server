import {LogEntryWithApplication} from "@/interfaces/LogEntry";

export interface LogTableResponse {
  logs: LogEntryWithApplication[];
  total_items: number;
}
