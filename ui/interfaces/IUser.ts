export interface IUser {
    uid: string;
    name: string;
    role: string;
    email: string;
    email_verified: boolean;
    is_active: boolean;
    created_at: {
        $date: {
            $numberLong: string;
        };
    };
    updated_at: {
        $date: {
            $numberLong: string;
        };
    };
}
