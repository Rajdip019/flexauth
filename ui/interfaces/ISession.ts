export interface ISession {
    session_id: string;
    email: string;
    user_agent: string;
    is_revoked: boolean;
    created_at: DateRecord;
    updated_at: DateRecord;
}

export interface DateRecord {
    $date: {
        $numberLong: string;
    };
}