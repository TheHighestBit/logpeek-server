export interface LogEntry {
  index: number;
  timestamp: string;
  level: string;
  module: string;
  message: string;
}

export interface LogEntryWithApplication {
  entry: LogEntry;
  application: string;
}
