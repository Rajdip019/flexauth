export interface IOverview {
    user_count: number;
    active_user_count: number;
    inactive_user_count: number;
    blocked_user_count: number;
    revoked_session_count: number;
    active_session_count: number;
    os_types: string[];
    device_types: string[];
    browser_types: string[];
}