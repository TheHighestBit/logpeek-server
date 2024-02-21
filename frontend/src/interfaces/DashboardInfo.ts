export interface DashboardInfo {
    total_logs_24: number[];
    error_logs_24: number[];
    warning_logs_24: number[];
    total_logs_week: number[];
    error_logs_week: number[];
    warning_logs_week: number[];
    top_modules: [string, number][];
    log_buffer_usage: number;
}
