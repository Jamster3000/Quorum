-- User Status
DEFINE TABLE user_status SCHEMAFULL;

DEFINE FIELD status ON user_status TYPE string ASSERT string::len($value) >= 2 AND string::len($value) <= 15;

DEFINE INDEX idx_status ON TABLE user_status COLUMNS status UNIQUE;


-- Custom Status
DEFINE TABLE custom_status SCHEMAFULL;

DEFINE FIELD status_text ON custom_status TYPE string ASSERT string::len($value) >= 2 AND string::len($value) <= 45;
DEFINE FIELD user_status_id ON custom_status TYPE option<record<user_status>>;

DEFINE INDEX idx_status_text ON TABLE custom_status COLUMNS status_text UNIQUE;


-- Users
DEFINE TABLE users SCHEMAFULL;

DEFINE FIELD username ON users TYPE string 
    ASSERT string::len($value) > 0 AND string::len($value) <= 50;
DEFINE FIELD email ON users TYPE string 
    ASSERT string::is_email($value) AND string::len($value) <= 100;
DEFINE FIELD password_hash ON users TYPE option<string>;
DEFINE FIELD avatar_url ON users TYPE option<string> 
    ASSERT $value == NONE OR string::len($value) <= 255;
DEFINE FIELD last_ip ON users TYPE option<string> 
    ASSERT $value == NONE OR string::len($value) <= 45;
DEFINE FIELD created_at ON users TYPE datetime DEFAULT time::now();
DEFINE FIELD last_login ON users TYPE option<datetime>;
DEFINE FIELD failed_login_attempts ON users TYPE number DEFAULT 0;
DEFINE FIELD account_locked_until ON users TYPE option<datetime>;
DEFINE FIELD two_factor_secret ON users TYPE option<string>;
DEFINE FIELD backup_codes ON users TYPE option<array<string>>;
DEFINE FIELD ban_expiry ON users TYPE option<datetime>;
DEFINE FIELD is_banned ON users TYPE bool DEFAULT false;
DEFINE FIELD is_deleted ON users TYPE bool DEFAULT false;
DEFINE FIELD scheduled_deletion_at ON users TYPE option<datetime>;
DEFINE FIELD user_status_id ON users TYPE option<record<user_status>>;
DEFINE FIELD custom_status_id ON users TYPE option<record<custom_status>>;

DEFINE INDEX idx_username ON TABLE users COLUMNS username UNIQUE;
DEFINE INDEX idx_email ON TABLE users COLUMNS email UNIQUE;


-- User settings
DEFINE TABLE user_settings SCHEMAFULL;

DEFINE FIELD user_id on user_settings TYPE record<users>;
DEFINE FIELD settings ON user_settings TYPE number DEFAULT 0;

DEFINE INDEX idx_user_id ON TABLE user_settings COLUMNS user_id UNIQUE;


-- Users custom emojis
DEFINE TABLE user_custom_emojis SCHEMAFULL;

DEFINE FIELD user_id on user_custom_emojis TYPE record<users>;
DEFINE FIELD name on user_custom_emojis TYPE string ASSERT string::len($value) > 0 AND string::len($value) <= 50;
DEFINE FIELD url on user_custom_emojis TYPE string ASSERT string::len($value) > 0 AND string::len($value) <= 255;
DEFINE FIELD uploaded_at on user_custom_emojis TYPE datetime DEFAULT time::now();

DEFINE INDEX idx_url ON TABLE user_custom_emojis COLUMNS url UNIQUE;


-- notification types
DEFINE TABLE notification_types SCHEMAFULL;

DEFINE FIELD type_name ON notification_types TYPE string 
    ASSERT string::len($value) > 0 AND string::len($value) <= 50;

DEFINE INDEX idx_type_name ON TABLE notification_types COLUMNS type_name UNIQUE;



-- notifications
DEFINE TABLE notifications SCHEMAFULL;

DEFINE FIELD user_id ON notifications TYPE record<users>;
DEFINE FIELD type ON notifications TYPE record<notification_types>;
DEFINE FIELD content ON notifications TYPE string 
    ASSERT string::len($value) > 0;
DEFINE FIELD is_read ON notifications TYPE bool DEFAULT false;
DEFINE FIELD created_at ON notifications TYPE datetime DEFAULT time::now();
DEFINE FIELD reference_type ON notifications TYPE option<string> 
    ASSERT $value == NONE OR string::len($value) <= 50;
DEFINE FIELD reference_id ON notifications TYPE option<number>;

DEFINE INDEX idx_notifications_user ON TABLE notifications COLUMNS user_id;
DEFINE INDEX idx_notifications_type ON TABLE notifications COLUMNS type;
DEFINE INDEX idx_notifications_created ON TABLE notifications COLUMNS created_at;


-- log type
DEFINE TABLE log_type SCHEMAFULL;

DEFINE FIELD name ON log_type TYPE string ASSERT string::len($value) > 0 AND string::len($value) <= 50;

DEFINE INDEX idx_name ON TABLE log_type COLUMNS name UNIQUE;


-- audit logs
DEFINE TABLE audit_logs SCHEMAFULL;

DEFINE FIELD user_id ON audit_logs TYPE record<users>;
DEFINE FIELD action ON audit_logs TYPE string;
DEFINE FIELD target_type_table ON audit_logs TYPE string;
DEFINE FIELD log_type_id ON audit_logs TYPE record<log_type>;
DEFINE FIELD old_value ON audit_logs TYPE string;
DEFINE FIELD new_value ON audit_logs TYPE string;
DEFINE FIELD created_at ON audit_logs TYPE datetime DEFAULT time::now();

DEFINE INDEX idx_audit_user ON TABLE audit_logs COLUMNS user_id;
DEFINE INDEX idx_audit_created ON TABLE audit_logs COLUMNS created_at;
DEFINE INDEX idx_audit_type ON TABLE audit_logs COLUMNS log_type_id;

-- access
DEFINE ACCESS user_access ON DATABASE TYPE RECORD
SIGNUP (
    CREATE users SET
        email = $email,
        username = $username,
        password_hash = crypto::argon2::generate($password),
        created_at = time::now()
)
SIGNIN (
    SELECT * FROM users 
    WHERE email = $email 
    AND crypto::argon2::compare(password_hash, $password)
    AND is_banned = false
    AND is_deleted = false
)
DURATION FOR SESSION 1d;