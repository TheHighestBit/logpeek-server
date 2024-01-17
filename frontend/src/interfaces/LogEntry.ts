export interface LogEntry {
  index: number;
  timestamp: number[];
  timestamp_formatted: string,
  level: string;
  module: string;
  message: string;
}
