export interface ISession {
    session_id: string;
    email: string;
    uid: string;
    user_agent: string;
    os: string;
    device: string;
    browser: string;
    browser_version: string;
    os_version: String,
    vendor: String,
    is_revoked: boolean;
    created_at: DateRecord;
    updated_at: DateRecord;
}

export interface DateRecord {
    $date: {
        $numberLong: string;
    };
}