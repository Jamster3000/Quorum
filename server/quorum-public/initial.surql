-- Global params
-- ==============
-- 64 bytes = 1/16 KiB
-- 128 bytes = 1/8 KiB
-- 256 bytes = 1/4 KiB
-- 512 bytes = 1/2 KiB
-- 1024 bytes = 1 KiB
-- 2048 bytes = 2 KiB
-- 4096 bytes = 4 KiB
-- 8192 bytes = 8 KiB
-- =========================================================
-- BASE / SHARED
-- =========================================================
DEFINE PARAM IF NOT EXISTS $MIN_NOTHING_BYTES VALUE 0;
DEFINE PARAM IF NOT EXISTS $MIN_DEFAULT_BYTES VALUE 1;
DEFINE PARAM IF NOT EXISTS $MIN_TAG_BYTES VALUE 2;
DEFINE PARAM IF NOT EXISTS $MIN_SPACE_NAME_BYTES VALUE 8;

DEFINE PARAM IF NOT EXISTS $MAX_TEXT_BYTES VALUE 4096;
DEFINE PARAM IF NOT EXISTS $MAX_URL_BYTES VALUE 128;
DEFINE PARAM IF NOT EXISTS $MAX_IP_BYTES VALUE 45;


-- =========================================================
-- USERS / ACCOUNT
-- =========================================================
DEFINE PARAM IF NOT EXISTS $MAX_USERNAME_BYTES VALUE 18;
DEFINE PARAM IF NOT EXISTS $MAX_EMAIL_BYTES VALUE 128;
DEFINE PARAM IF NOT EXISTS $MAX_USER_STATUS_BYTES VALUE 15;
DEFINE PARAM IF NOT EXISTS $MAX_EMOJI_CODE_BYTES VALUE 20;
DEFINE PARAM IF NOT EXISTS $MIN_USERNAME_BYTES VALUE 4;


-- =========================================================
-- TAGS / ROLES / TYPES
-- =========================================================
DEFINE PARAM IF NOT EXISTS $MAX_TAG_BYTES VALUE 32;
DEFINE PARAM IF NOT EXISTS $MAX_GROUP_ROLES_BYTES VALUE 32;
DEFINE PARAM IF NOT EXISTS $MAX_PROJECT_TYPE_BYTES VALUE 20;
DEFINE PARAM IF NOT EXISTS $MAX_FILE_TYPE_NAME_BYTES VALUE 50;


-- =========================================================
-- SPACES
-- =========================================================
DEFINE PARAM IF NOT EXISTS $MAX_SPACES_NAME_BYTES VALUE 128;
DEFINE PARAM IF NOT EXISTS $MAX_SPACE_DESCRIPTION_BYTES VALUE 1024;


-- =========================================================
-- FILES / FOLDERS / STORAGE
-- =========================================================
DEFINE PARAM IF NOT EXISTS $MAX_FILE_NAME_BYTES VALUE 50;
DEFINE PARAM IF NOT EXISTS $MAX_FOLDER_NAME_BYTES VALUE 50;
DEFINE PARAM IF NOT EXISTS $MAX_BUCKET_NAME_BYTES VALUE 64;
DEFINE PARAM IF NOT EXISTS $MAX_FILE_OBJECT_KEY_BYTES VALUE 512;
DEFINE PARAM IF NOT EXISTS $MAX_CHECKSUM_SHA256_BYTES VALUE 64;


-- =========================================================
-- NOTIFICATIONS / AUDIT / LOGGING
-- =========================================================
DEFINE PARAM IF NOT EXISTS $MAX_NOTIFICATION_TYPES_BYTES VALUE 40;
DEFINE PARAM IF NOT EXISTS $MAX_NOTIFICATION_REFERENCE_BYTES VALUE 30;
DEFINE PARAM IF NOT EXISTS $MAX_LOG_TYPE_BYTES VALUE 20;
DEFINE PARAM IF NOT EXISTS $MAX_AUDIT_ACTION_BYTES VALUE 128;
DEFINE PARAM IF NOT EXISTS $MAX_AUDIT_TARGET_BYTES VALUE 64;
DEFINE PARAM IF NOT EXISTS $MAX_AUDIT_VALUE_BYTES VALUE 512;


-- =========================================================
-- TABLE BOARD
-- =========================================================
DEFINE PARAM IF NOT EXISTS $MAX_TABLE_COLUMN_NAME_BYTES VALUE 15;
DEFINE PARAM IF NOT EXISTS $MAX_TABLE_CELL_BYTES VALUE 512;


-- =========================================================
-- GANTT
-- =========================================================
DEFINE PARAM IF NOT EXISTS $MAX_GANTT_TITLE_BYTES VALUE 128;
DEFINE PARAM IF NOT EXISTS $MAX_GANTT_DESCRIPTION_BYTES VALUE 1024;
DEFINE PARAM IF NOT EXISTS $MAX_GANTT_DEPENDENCIES_BYTES VALUE 512;


-- =========================================================
-- TODO
-- =========================================================
DEFINE PARAM IF NOT EXISTS $MAX_TODO_TITLE_BYTES VALUE 128;
DEFINE PARAM IF NOT EXISTS $MAX_TODO_NAME_BYTES VALUE 64;


-- =========================================================
-- CALENDAR
-- =========================================================
DEFINE PARAM IF NOT EXISTS $MAX_CALENDAR_TITLE_BYTES VALUE 128;
DEFINE PARAM IF NOT EXISTS $MAX_CALENDAR_DESCRIPTION_BYTES VALUE 1024;


-- =========================================================
-- KANBAN
-- =========================================================
DEFINE PARAM IF NOT EXISTS $MAX_KANBAN_COLUMN_NAME_BYTES VALUE 64;
DEFINE PARAM IF NOT EXISTS $MAX_KANBAN_TASK_TITLE_BYTES VALUE 128;
DEFINE PARAM IF NOT EXISTS $MAX_KANBAN_TASK_DESCRIPTION_BYTES VALUE 1024;


-- Functions
DEFINE FUNCTION OVERWRITE fn::bytes_within(
    $value: string,
    $min_bytes: number,
    $max_bytes: number
) -> bool {
    RETURN bytes::len(<bytes>$value) >= $min_bytes
       AND bytes::len(<bytes>$value) <= $max_bytes;
};


-- Server config
DEFINE TABLE IF NOT EXISTS server_config SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS first_admin_bootstrapped ON TABLE server_config TYPE bool DEFAULT false;

INSERT IGNORE INTO server_config { first_admin_bootstrapped: false };

-- User Status
DEFINE TABLE IF NOT EXISTS user_status SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS status ON TABLE user_status TYPE string ASSERT fn::bytes_within($value, $MIN_TEXT_BYTES, $MAX_USER_STATUS_BYTES);

DEFINE INDEX IF NOT EXISTS idx_status ON TABLE user_status COLUMNS status UNIQUE;


-- Custom Status
DEFINE TABLE IF NOT EXISTS custom_status SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS status_text ON TABLE custom_status TYPE string ASSERT fn::bytes_within($value, $MIN_DEFAULT_BYTES, $MAX_USER_STATUS_BYTES);
DEFINE FIELD IF NOT EXISTS user_status_id ON TABLE custom_status TYPE option<record<user_status>>;

DEFINE INDEX IF NOT EXISTS idx_status_text ON TABLE custom_status COLUMNS status_text UNIQUE;


-- Users
DEFINE TABLE IF NOT EXISTS users SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS username ON TABLE users TYPE string ASSERT fn::bytes_within($value, $MIN_USERNAME_BYTES, $MAX_USERNAME_BYTES);
DEFINE FIELD IF NOT EXISTS email ON TABLE users TYPE option<string> ASSERT fn::bytes_within($value, $MIN_NOTHING_BYTES, $MAX_EMAIL_BYTES);
DEFINE FIELD IF NOT EXISTS password_hash ON TABLE users TYPE string;
DEFINE FIELD IF NOT EXISTS is_admin ON TABLE users TYPE bool DEFAULT false;
DEFINE FIELD IF NOT EXISTS avatar_url ON TABLE users TYPE option<record<media>>;
DEFINE FIELD IF NOT EXISTS created_at ON TABLE users TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS last_login ON TABLE users TYPE option<datetime>;
DEFINE FIELD IF NOT EXISTS account_locked_until ON TABLE users TYPE option<datetime>;
DEFINE FIELD IF NOT EXISTS two_factor_secret ON TABLE users TYPE option<string>;
DEFINE FIELD IF NOT EXISTS email_backup_codes ON TABLE users TYPE option<array<string>>;
DEFINE FIELD IF NOT EXISTS ban_expiry ON TABLE users TYPE option<datetime>;
DEFINE FIELD IF NOT EXISTS is_banned ON TABLE users TYPE bool DEFAULT false;
DEFINE FIELD IF NOT EXISTS is_deleted ON TABLE users TYPE bool DEFAULT false;
DEFINE FIELD IF NOT EXISTS scheduled_deletion_at ON TABLE users TYPE option<datetime>;
DEFINE FIELD IF NOT EXISTS user_status_id ON TABLE users TYPE option<record<user_status>>;
DEFINE FIELD IF NOT EXISTS custom_status_id ON TABLE users TYPE option<record<custom_status>>;

DEFINE INDEX IF NOT EXISTS idx_username ON TABLE users COLUMNS username UNIQUE;
DEFINE INDEX IF NOT EXISTS idx_email ON TABLE users COLUMNS email UNIQUE;


-- refresh tokens
DEFINE TABLE IF NOT EXISTS refresh_tokens SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS user_id ON TABLE refresh_tokens TYPE record<users>;
DEFINE FIELD IF NOT EXISTS token_hash ON TABLE refresh_tokens TYPE string;
DEFINE FIELD IF NOT EXISTS expires_at ON TABLE refresh_tokens TYPE datetime;
DEFINE FIELD IF NOT EXISTS created_at ON TABLE refresh_tokens TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS is_revoked ON TABLE refresh_tokens TYPE bool DEFAULT false;

DEFINE INDEX IF NOT EXISTS idx_refresh_token_user ON TABLE refresh_tokens COLUMNS user_id;
DEFINE INDEX IF NOT EXISTS idx_refresh_token_hash ON TABLE refresh_tokens COLUMNS token_hash UNIQUE;


-- User settings
DEFINE TABLE IF NOT EXISTS user_settings SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS user_id on TABLE user_settings TYPE record<users>;
DEFINE FIELD IF NOT EXISTS settings ON TABLE user_settings TYPE number DEFAULT 0;

DEFINE INDEX IF NOT EXISTS idx_user_id ON TABLE user_settings COLUMNS user_id UNIQUE;


-- Users custom emojis
DEFINE TABLE IF NOT EXISTS user_custom_emojis SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS user_id on TABLE user_custom_emojis TYPE record<users>;
DEFINE FIELD IF NOT EXISTS name on TABLE user_custom_emojis TYPE string ASSERT fn::bytes_within($value, $MIN_DEFAULT_BYTES, $MAX_USERNAME_BYTES);
DEFINE FIELD IF NOT EXISTS url on TABLE user_custom_emojis TYPE record<media>;
DEFINE FIELD IF NOT EXISTS uploaded_at on TABLE user_custom_emojis TYPE datetime DEFAULT time::now();

DEFINE INDEX IF NOT EXISTS idx_url ON TABLE user_custom_emojis COLUMNS url UNIQUE;


-- notification types
DEFINE TABLE IF NOT EXISTS notification_types SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS type_name ON TABLE notification_types TYPE string
    ASSERT fn::bytes_within($value, $MIN_DEFAULT_BYTES, $MAX_NOTIFICATION_TYPES_BYTES);

DEFINE INDEX IF NOT EXISTS idx_type_name ON TABLE notification_types COLUMNS type_name UNIQUE;


-- notifications
DEFINE TABLE IF NOT EXISTS notifications SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS user_id ON TABLE notifications TYPE record<users>;
DEFINE FIELD IF NOT EXISTS type ON TABLE notifications TYPE record<notification_types>;
DEFINE FIELD IF NOT EXISTS content ON TABLE notifications TYPE object; --{"type": "", "reaction": "", "message_id": "", "space_id": "", "group_id": ""} As an example
DEFINE FIELD IF NOT EXISTS is_read ON TABLE notifications TYPE bool DEFAULT false;
DEFINE FIELD IF NOT EXISTS created_at ON TABLE notifications TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS reference_type ON TABLE notifications TYPE option<string> ASSERT $value == NONE OR fn::bytes_within($value, $MIN_DEFAULT_BYTES, $MAX_NOTIFICATION_REFERENCE_BYTES);
DEFINE FIELD IF NOT EXISTS reference_id ON TABLE notifications TYPE option<number>;

DEFINE INDEX IF NOT EXISTS idx_notifications_user ON TABLE notifications COLUMNS user_id;
DEFINE INDEX IF NOT EXISTS idx_notifications_type ON TABLE notifications COLUMNS type;


-- log type
DEFINE TABLE IF NOT EXISTS log_type SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS name ON TABLE log_type TYPE string ASSERT fn::bytes_within($value, $MIN_DEFAULT_BYTES, $MAX_LOG_TYPE_BYTES);

DEFINE INDEX IF NOT EXISTS idx_name ON TABLE log_type COLUMNS name UNIQUE;


-- audit action type
DEFINE TABLE IF NOT EXISTS audit_action_type SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS name ON TABLE audit_action_type TYPE string ASSERT fn::bytes_within($value, $MIN_DEFAULT_BYTES, $MAX_AUDIT_ACTION_BYTES);

DEFINE INDEX IF NOT EXISTS idx_audit_action_type_name ON TABLE audit_action_type COLUMNS name UNIQUE;

-- audit logs
DEFINE TABLE IF NOT EXISTS audit_logs SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS user_id ON TABLE audit_logs TYPE option<record<users>>;
DEFINE FIELD IF NOT EXISTS action_type_id ON TABLE audit_logs TYPE option<record<audit_action_type>>;
DEFINE FIELD IF NOT EXISTS target_type_table_id ON TABLE audit_logs TYPE option<record>;
DEFINE FIELD IF NOT EXISTS log_type_id ON TABLE audit_logs TYPE record<log_type>;
DEFINE FIELD IF NOT EXISTS old_value ON TABLE audit_logs TYPE option<string> ASSERT $value = NONE OR fn::bytes_within($value, $MIN_DEFAULT_BYTES, $MAX_AUDIT_VALUE_BYTES);
DEFINE FIELD IF NOT EXISTS new_value ON TABLE audit_logs TYPE option<string> ASSERT $value = NONE OR fn::bytes_within($value, $MIN_DEFAULT_BYTES, $MAX_AUDIT_VALUE_BYTES);
DEFINE FIELD IF NOT EXISTS created_at ON TABLE audit_logs TYPE datetime DEFAULT time::now();

DEFINE INDEX IF NOT EXISTS idx_audit_user ON TABLE audit_logs COLUMNS user_id;
DEFINE INDEX IF NOT EXISTS idx_audit_created ON TABLE audit_logs COLUMNS created_at;
DEFINE INDEX IF NOT EXISTS idx_audit_type ON TABLE audit_logs COLUMNS log_type_id;


-- server log event types
DEFINE TABLE IF NOT EXISTS server_log_event_types SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS name ON TABLE server_log_event_types TYPE string;

DEFINE INDEX IF NOT EXISTS idx_server_log_event_types_name ON TABLE server_log_event_types COLUMNS name UNIQUE;


-- Server logs
DEFINE TABLE IF NOT EXISTS server_logs SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS event_type_id ON TABLE server_logs TYPE record<server_log_event_types>;
DEFINE FIELD IF NOT EXISTS timestamp ON TABLE server_logs type datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS message ON TABLE server_logs TYPE option<string>;
DEFINE FIELD IF NOT EXISTS error_code ON TABLE server_logs TYPE option<number>;
DEFINE FIELD IF NOT EXISTS duration_ms ON TABLE server_logs TYPE option<number>;

DEFINE INDEX IF NOT EXISTS idx_server_logs_timestamp ON TABLE server_logs COLUMNS timestamp;
DEFINE INDEX IF NOT EXISTS idx_server_logs_event ON TABLE server_logs COLUMNS event_type_id;


-- Direct messages
DEFINE TABLE IF NOT EXISTS direct_messages SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS sender_id ON TABLE direct_messages TYPE record<users>;
DEFINE FIELD IF NOT EXISTS receiver_id ON TABLE direct_messages TYPE record<users>;
DEFINE FIELD IF NOT EXISTS content ON TABLE direct_messages TYPE string ASSERT fn::bytes_within($value, $MIN_DEFAULT_BYTES, $MAX_TEXT_BYTES);
DEFINE FIELD IF NOT EXISTS created_at ON TABLE direct_messages TYPE datetime;
DEFINE FIELD IF NOT EXISTS is_read ON TABLE direct_messages TYPE bool DEFAULT false;
DEFINE FIELD IF NOT EXISTS attatchments ON TABLE direct_messages TYPE option<array> DEFAULT NONE;
DEFINE FIELD IF NOT EXISTS is_deleted ON TABLE direct_messages TYPE bool DEFAULT false;
DEFINE FIELD OVERWRITE deleted_at ON TABLE direct_messages TYPE option<datetime>;

DEFINE INDEX IF NOT EXISTS idx_sender_receiver ON TABLE direct_messages COLUMNS sender_id, receiver_id;
DEFINE INDEX IF NOT EXISTS idx_receiver_sender ON TABLE direct_messages COLUMNS receiver_id, sender_id;
DEFINE INDEX IF NOT EXISTS idx_created_at ON TABLE direct_messages COLUMNS created_at;
DEFINE INDEX IF NOT EXISTS idx_is_read ON TABLE direct_messages COLUMNS is_read;
DEFINE INDEX IF NOT EXISTS idx_is_deleted ON TABLE direct_messages COLUMNS is_deleted;
DEFINE INDEX IF NOT EXISTS idx_deleted_at ON TABLE direct_messages COLUMNS deleted_at;


-- emojis
DEFINE TABLE IF NOT EXISTS emojis SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS code ON TABLE emojis TYPE string ASSERT fn::bytes_within($value, $MIN_DEFAULT_BYTES, $MAX_EMOJI_CODE_BYTES);


-- message reactions
DEFINE TABLE IF NOT EXISTS message_reactions SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS message_id ON TABLE message_reactions TYPE record<direct_messages | group_messages>;
DEFINE FIELD IF NOT EXISTS emoji_id ON TABLE message_reactions TYPE option<record<emojis>>;
DEFINE FIELD IF NOT EXISTS custom_emoji_id ON TABLE message_reactions TYPE option<record<user_custom_emojis>>;
DEFINE FIELD IF NOT EXISTS count ON TABLE message_reactions TYPE number;

DEFINE INDEX IF NOT EXISTS idx_message_reaction ON TABLE message_reactions COLUMNS message_id;
DEFINE INDEX IF NOT EXISTS idx_reaction_emoji ON TABLE message_reactions COLUMNS emoji_id;
DEFINE INDEX IF NOT EXISTS idx_reaction_custom_emoji ON TABLE message_reactions COLUMNS custom_emoji_id;


-- space types
DEFINE TABLE IF NOT EXISTS space_type SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS name ON TABLE space_type TYPE string ASSERT fn::bytes_within($value, $MIN_SPACE_NAME_BYTES, $MAX_LOG_TYPE_BYTES);


-- group tags
DEFINE TABLE IF NOT EXISTS group_tags SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS tag ON TABLE group_tags TYPE string ASSERT fn::bytes_within($value, $MIN_TAG_BYTES, $MAX_TAG_BYTES);

DEFINE INDEX IF NOT EXISTS idx_group_tags_tag ON TABLE group_tags COLUMNS tag UNIQUE;


-- Groups
DEFINE TABLE IF NOT EXISTS groups SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS name ON TABLE groups TYPE string ASSERT fn::bytes_within($value, $MIN_SPACE_NAME_BYTES, $MAX_SPACES_NAME_BYTES);
DEFINE FIELD IF NOT EXISTS description ON TABLE groups TYPE string ASSERT fn::bytes_within($value, $MIN_NOTHING_BYTES, $MAX_SPACE_DESCRIPTION_BYTES);
DEFINE FIELD IF NOT EXISTS user_id ON TABLE groups TYPE record<users>;
DEFINE FIELD IF NOT EXISTS group_icon_url ON TABLE groups TYPE record<media>;
DEFINE FIELD IF NOT EXISTS privacy ON TABLE groups TYPE bool DEFAULT 0;
DEFINE FIELD IF NOT EXISTS group_tags_id ON TABLE groups TYPE option<record<group_tags>>;

DEFINE INDEX IF NOT EXISTS idx_group_name ON TABLE groups COLUMNS name UNIQUE;
DEFINE INDEX IF NOT EXISTS idx_group_user ON TABLE groups COLUMNS user_id;
DEFINE INDEX IF NOT EXISTS idx_groups_tag ON TABLE groups COLUMNS group_tags_id;
DEFINE INDEX IF NOT EXISTS idx_groups_privacy ON TABLE groups COLUMNS privacy;


-- banned group members
DEFINE TABLE IF NOT EXISTS group_members_banned SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS group_id ON TABLE group_members_banned TYPE record<groups>;
DEFINE FIELD IF NOT EXISTS user_id ON TABLE group_members_banned TYPE record<users>;
DEFINE FIELD IF NOT EXISTS reason ON TABLE group_members_banned TYPE string ASSERT $value == NONE OR fn::bytes_within($value, $MIN_NOTHING_BYTES, $MAX_BANNED_BYTES);
DEFINE FIELD IF NOT EXISTS created_at ON TABLE group_members_banned TYPE datetime;

DEFINE INDEX IF NOT EXISTS idx_group_members_banned_group_user
ON TABLE group_members_banned COLUMNS group_id, user_id UNIQUE;
DEFINE INDEX IF NOT EXISTS idx_group_members_banned_user
ON TABLE group_members_banned COLUMNS user_id;
DEFINE INDEX IF NOT EXISTS idx_group_members_banned_created_at
ON TABLE group_members_banned COLUMNS created_at;


-- group members
DEFINE TABLE IF NOT EXISTS group_members SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS group_id ON TABLE group_members TYPE record<groups>;
DEFINE FIELD IF NOT EXISTS user_id ON TABLE group_members TYPE record<users>;
DEFINE FIELD IF NOT EXISTS role_id ON TABLE group_members TYPE record<group_roles>;
DEFINE FIELD IF NOT EXISTS permissions_cache ON TABLE group_members TYPE number;
DEFINE FIELD IF NOT EXISTS joined_at ON TABLE group_members TYPE datetime;
DEFINE FIELD IF NOT EXISTS banned_id ON TABLE group_members TYPE option<record<group_members_banned>>;
DEFINE FIELD IF NOT EXISTS is_timeout ON TABLE group_members TYPE bool DEFAULT false;
DEFINE FIELD IF NOT EXISTS expiry ON TABLE group_members TYPE datetime;

DEFINE INDEX IF NOT EXISTS idx_group_members_group_user
ON TABLE group_members COLUMNS group_id, user_id UNIQUE;
DEFINE INDEX IF NOT EXISTS idx_group_members_user
ON TABLE group_members COLUMNS user_id;
DEFINE INDEX IF NOT EXISTS idx_group_members_role
ON TABLE group_members COLUMNS role_id;


-- group roles
DEFINE TABLE IF NOT EXISTS group_roles SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS group_id ON TABLE group_roles TYPE record<groups>;
DEFINE FIELD IF NOT EXISTS name ON TABLE group_roles TYPE string ASSERT fn::bytes_within($value, $MIN_DEFAULT_BYTES, $MAX_GROUP_ROLES_BYTES);
DEFINE FIELD IF NOT EXISTS colour ON TABLE group_roles TYPE number ASSERT $value >= 0 AND $value <= 16777215;
DEFINE FIELD IF NOT EXISTS position ON TABLE group_roles TYPE number DEFAULT 0;
DEFINE FIELD IF NOT EXISTS permissions ON TABLE group_roles TYPE number;

DEFINE INDEX IF NOT EXISTS idx_group_roles_group_position
ON TABLE group_roles COLUMNS group_id, position;

DEFINE INDEX IF NOT EXISTS idx_group_roles_group_name
ON TABLE group_roles COLUMNS group_id, name UNIQUE;


-- group settings
DEFINE TABLE IF NOT EXISTS group_settings SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS group_id ON TABLE group_settings TYPE record<groups>;
DEFINE FIELD IF NOT EXISTS settings ON TABLE group_settings TYPE number;

DEFINE INDEX IF NOT EXISTS idx_settings ON TABLE group_settings COLUMNS settings;


-- spaces
DEFINE TABLE IF NOT EXISTS spaces SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS group_id ON TABLE spaces TYPE record<groups>;
DEFINE FIELD IF NOT EXISTS name ON TABLE spaces TYPE string ASSERT fn::bytes_within($value, $MIN_SPACE_NAME_BYTES, $MAX_SPACES_NAME_BYTES);
DEFINE FIELD IF NOT EXISTS space_type_id ON TABLE spaces TYPE record<space_type>;
DEFINE FIELD IF NOT EXISTS description ON TABLE spaces TYPE string ASSERT fn::bytes_within($value, $MIN_NOTHING_BYTES, $MAX_SPACE_DESCRIPTION_BYTES);
DEFINE FIELD IF NOT EXISTS created_at ON TABLE spaces TYPE datetime;

DEFINE INDEX IF NOT EXISTS idx_space_group ON TABLE spaces COLUMNS group_id;
DEFINE INDEX IF NOT EXISTS idx_spaces_type ON TABLE spaces COLUMNS space_type_id;
DEFINE INDEX IF NOT EXISTS idx_spaces_created_at ON TABLE spaces COLUMNS created_at;


-- messages
DEFINE TABLE IF NOT EXISTS group_messages SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS space_id ON TABLE group_messages TYPE record<spaces>;
DEFINE FIELD IF NOT EXISTS user_id ON TABLE group_messages TYPE record<users>;
DEFINE FIELD IF NOT EXISTS content ON TABLE group_messages TYPE string ASSERT fn::bytes_within($value, $MIN_DEFAULT_BYTES, $MAX_TEXT_BYTES);
DEFINE FIELD IF NOT EXISTS created_at ON TABLE group_messages TYPE option<datetime>;
DEFINE FIELD IF NOT EXISTS edited_at ON TABLE group_messages TYPE option<datetime>;
DEFINE FIELD IF NOT EXISTS is_pinned ON TABLE group_messages TYPE bool DEFAULT false;
DEFINE FIELD IF NOT EXISTS message_id ON TABLE group_messages TYPE option<record<group_messages>>;
DEFINE FIELD IF NOT EXISTS attatchments ON TABLE group_messages TYPE option<array> DEFAULT NONE;
DEFINE FIELD IF NOT EXISTS is_deleted ON TABLE group_messages TYPE bool DEFAULT false;
DEFINE FIELD IF NOT EXISTS deleted_at ON TABLE group_messages TYPE datetime;

DEFINE INDEX IF NOT EXISTS idx_group_messages_space_created ON TABLE group_messages COLUMNS space_id, created_at;
DEFINE INDEX IF NOT EXISTS idx_group_messages_user ON TABLE group_messages COLUMNS user_id;
DEFINE INDEX IF NOT EXISTS idx_group_messages_message_id ON TABLE group_messages COLUMNS message_id;


-- file_type
DEFINE TABLE IF NOT EXISTS file_type SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS name
ON TABLE file_type TYPE string
ASSERT fn::bytes_within($value, $MIN_DEFAULT_BYTES, $MAX_FILE_TYPE_NAME_BYTES);

DEFINE INDEX IF NOT EXISTS idx_file_type_name_unique
ON TABLE file_type COLUMNS name UNIQUE;


-- media
DEFINE TABLE IF NOT EXISTS media SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS space_id ON TABLE media TYPE option<record<spaces>>;
DEFINE FIELD IF NOT EXISTS user_id ON TABLE media TYPE record<users>;
DEFINE FIELD IF NOT EXISTS url ON TABLE media TYPE string ASSERT fn::bytes_within($value, $MIN_DEFAULT_BYTES, $MAX_URL_BYTES);
DEFINE FIELD IF NOT EXISTS file_mime_type_id ON TABLE media TYPE record<file_type>;
DEFINE FIELD IF NOT EXISTS uploaded_at ON TABLE media TYPE datetime;
DEFINE FIELD IF NOT EXISTS width ON TABLE media TYPE option<number> ASSERT $value == NONE OR $value > 0;
DEFINE FIELD IF NOT EXISTS height ON TABLE media TYPE option<number> ASSERT $value == NONE OR $value > 0;
DEFINE FIELD IF NOT EXISTS thumbnail_url ON TABLE media TYPE string ASSERT fn::bytes_within($value, $MIN_DEFAULT_BYTES, $MAX_URL_BYTES);

DEFINE INDEX IF NOT EXISTS idx_media_space_uploaded ON TABLE media COLUMNS space_id, uploaded_at;
DEFINE INDEX IF NOT EXISTS idx_media_user_uploaded ON TABLE media COLUMNS user_id, uploaded_at;


-- project type
DEFINE TABLE IF NOT EXISTS project_type SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS type ON TABLE project_type TYPE string ASSERT fn::bytes_within($value, $MIN_DEFAULT_BYTES, $MAX_PROJECT_TYPE_BYTES);


-- Project board
DEFINE TABLE IF NOT EXISTS project_board SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS space_id ON TABLE project_board TYPE record<spaces>;
DEFINE FIELD IF NOT EXISTS project_type_id ON TABLE project_board TYPE record<project_type>;

DEFINE INDEX IF NOT EXISTS idx_space ON TABLE project_board COLUMNS space_id;


-- folders
DEFINE TABLE IF NOT EXISTS folders SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS space_id ON TABLE folders TYPE record<spaces>;
DEFINE FIELD IF NOT EXISTS parent_folder_id ON TABLE folders TYPE option<record<folders>>;
DEFINE FIELD IF NOT EXISTS name ON TABLE folders TYPE string ASSERT fn::bytes_within($value, $MIN_DEFAULT_BYTES, $MAX_FOLDER_NAME_BYTES);
DEFINE FIELD IF NOT EXISTS created_at ON TABLE folders TYPE datetime;
DEFINE FIELD IF NOT EXISTS updated_at ON TABLE folders TYPE option<datetime>;
DEFINE FIELD IF NOT EXISTS created_by ON TABLE folders TYPE record<users>;

DEFINE INDEX IF NOT EXISTS idx_folders_space_parent ON TABLE folders COLUMNS space_id, parent_folder_id;
DEFINE INDEX IF NOT EXISTS idx_folders_created_by ON TABLE folders COLUMNS created_by;


-- files
DEFINE TABLE IF NOT EXISTS files SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS folder_id ON TABLE files TYPE option<record<folders>>;
DEFINE FIELD IF NOT EXISTS space_id ON TABLE files TYPE record<spaces>;
DEFINE FIELD IF NOT EXISTS name ON TABLE files TYPE string ASSERT fn::bytes_within($value, $MIN_DEFAULT_BYTES, $MAX_FILE_NAME_BYTES);
DEFINE FIELD IF NOT EXISTS bucket ON TABLE files TYPE string ASSERT fn::bytes_within($value, $MIN_DEFAULT_BYTES, $MAX_BUCKET_NAME_BYTES);
DEFINE FIELD IF NOT EXISTS object_key ON TABLE files TYPE string ASSERT fn::bytes_within($value, $MIN_DEFAULT_BYTES, $MAX_FILE_OBJECT_KEY_BYTES);
DEFINE FIELD IF NOT EXISTS size_bytes ON TABLE files TYPE int ASSERT $value >= 0;
DEFINE FIELD IF NOT EXISTS file_type_id ON TABLE files TYPE record<file_type>;
DEFINE FIELD IF NOT EXISTS checksum_sha256 ON TABLE files TYPE option<string> ASSERT $value = NONE OR fn::bytes_within($value, $MIN_DEFAULT_BYTES, $MAX_CHECKSUM_SHA256_BYTES);
DEFINE FIELD IF NOT EXISTS created_at ON TABLE files TYPE datetime;
DEFINE FIELD IF NOT EXISTS updated_at ON TABLE files TYPE option<datetime>;
DEFINE FIELD IF NOT EXISTS created_by ON TABLE files TYPE record<users>;

DEFINE INDEX IF NOT EXISTS idx_files_space_folder ON TABLE files COLUMNS space_id, folder_id;
DEFINE INDEX IF NOT EXISTS idx_files_created_by ON TABLE files COLUMNS created_by;
DEFINE INDEX IF NOT EXISTS idx_files_file_type ON TABLE files COLUMNS file_type_id;
DEFINE INDEX IF NOT EXISTS idx_files_bucket_object_key_unique ON TABLE files COLUMNS bucket, object_key UNIQUE;


-- file_permissions
DEFINE TABLE IF NOT EXISTS file_permissions SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS space_id ON TABLE file_permissions TYPE record<spaces>;
DEFINE FIELD IF NOT EXISTS role_id ON TABLE file_permissions TYPE option<record<group_roles>>;
DEFINE FIELD IF NOT EXISTS folder_id ON TABLE file_permissions TYPE option<record<folders>>;
DEFINE FIELD IF NOT EXISTS file_id ON TABLE file_permissions TYPE option<record<files>>;
DEFINE FIELD IF NOT EXISTS user_id ON TABLE file_permissions TYPE option<record<users>>;
DEFINE FIELD IF NOT EXISTS permissions ON TABLE file_permissions TYPE int ASSERT $value >= 0;

DEFINE INDEX IF NOT EXISTS idx_file_permissions_space ON TABLE file_permissions COLUMNS space_id;
DEFINE INDEX IF NOT EXISTS idx_file_permissions_folder ON TABLE file_permissions COLUMNS folder_id;
DEFINE INDEX IF NOT EXISTS idx_file_permissions_file ON TABLE file_permissions COLUMNS file_id;
DEFINE INDEX IF NOT EXISTS idx_file_permissions_user ON TABLE file_permissions COLUMNS user_id;
DEFINE INDEX IF NOT EXISTS idx_file_permissions_role ON TABLE file_permissions COLUMNS role_id;


-- project board rows
DEFINE TABLE IF NOT EXISTS table_board_rows SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS board_id ON TABLE table_board_rows TYPE record<project_board>;
DEFINE FIELD IF NOT EXISTS position ON TABLE table_board_rows TYPE int ASSERT $value >= 0;

DEFINE INDEX IF NOT EXISTS idx_table_board_rows_board ON TABLE table_board_rows COLUMNS board_id;
DEFINE INDEX IF NOT EXISTS idx_table_board_rows_position ON TABLE table_board_rows COLUMNS board_id, position;


-- Project board columns
DEFINE TABLE IF NOT EXISTS table_board_columns SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS board_id ON TABLE table_board_columns TYPE record<project_board>;
DEFINE FIELD IF NOT EXISTS name ON TABLE table_board_columns TYPE string ASSERT fn::bytes_within($value, $MIN_DEFAULT_BYTES, $MAX_TABLE_COLUMN_NAME_BYTES);

DEFINE INDEX IF NOT EXISTS idx_table_board_columns_board ON TABLE table_board_columns COLUMNS board_id;


-- project board cells
DEFINE TABLE IF NOT EXISTS table_board_cells SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS row_id ON TABLE table_board_cells TYPE record<table_board_rows>;
DEFINE FIELD IF NOT EXISTS column_id ON TABLE table_board_cells TYPE record<table_board_columns>;
DEFINE FIELD IF NOT EXISTS value ON TABLE table_board_cells TYPE string ASSERT fn::bytes_within($value, $MIN_DEFAULT_BYTES, $MAX_TABLE_CELL_BYTES);

DEFINE INDEX IF NOT EXISTS idx_table_board_cells_row ON TABLE table_board_cells COLUMNS row_id;
DEFINE INDEX IF NOT EXISTS idx_table_board_cells_column ON TABLE table_board_cells COLUMNS column_id;
DEFINE INDEX IF NOT EXISTS idx_table_board_cells_row_column ON TABLE table_board_cells COLUMNS row_id, column_id UNIQUE;


-- Gantt chart
DEFINE TABLE IF NOT EXISTS gantt_task SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS board_id ON TABLE gantt_task TYPE record<project_board>;
DEFINE FIELD IF NOT EXISTS title ON TABLE gantt_task TYPE string ASSERT fn::bytes_within($value, $MIN_DEFAULT_BYTES, $MAX_GANTT_TITLE_BYTES);
DEFINE FIELD IF NOT EXISTS description ON TABLE gantt_task TYPE option<string> ASSERT $value = NONE OR fn::bytes_within($value, $MIN_NOTHING_BYTES, $MAX_GANTT_DESCRIPTION_BYTES);
DEFINE FIELD IF NOT EXISTS start_date ON TABLE gantt_task TYPE datetime;
DEFINE FIELD IF NOT EXISTS end_date ON TABLE gantt_task TYPE datetime ASSERT $value > start_date;
DEFINE FIELD IF NOT EXISTS dependencies ON TABLE gantt_task TYPE array<string> ASSERT array::len($value) <= 20;

DEFINE INDEX IF NOT EXISTS idx_gantt_task_board ON TABLE gantt_task COLUMNS board_id;
DEFINE INDEX IF NOT EXISTS idx_gantt_task_date_range ON TABLE gantt_task COLUMNS board_id, start_date, end_date;


-- ToDo Tasks table
DEFINE TABLE IF NOT EXISTS todo_tasks SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS board_id ON TABLE todo_tasks TYPE record<project_board>;
DEFINE FIELD IF NOT EXISTS title ON TABLE todo_tasks TYPE string ASSERT fn::bytes_within($value, $MIN_DEFAULT_BYTES, $MAX_TODO_TITLE_BYTES);
DEFINE FIELD IF NOT EXISTS is_complete ON TABLE todo_tasks TYPE bool DEFAULT false;
DEFINE FIELD IF NOT EXISTS due_date ON TABLE todo_tasks TYPE option<datetime>;
DEFINE FIELD IF NOT EXISTS created_at ON TABLE todo_tasks TYPE datetime;
DEFINE FIELD IF NOT EXISTS updated_at ON TABLE todo_tasks TYPE datetime;
DEFINE FIELD IF NOT EXISTS created_by ON TABLE todo_tasks TYPE record<users>;

DEFINE INDEX IF NOT EXISTS idx_todo_tasks_board ON TABLE todo_tasks COLUMNS board_id;
DEFINE INDEX IF NOT EXISTS idx_todo_tasks_complete ON TABLE todo_tasks COLUMNS board_id, is_complete;
DEFINE INDEX IF NOT EXISTS idx_todo_tasks_due ON TABLE todo_tasks COLUMNS board_id, due_date;


-- Priority table
DEFINE TABLE IF NOT EXISTS priority SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS name ON TABLE priority TYPE string ASSERT fn::bytes_within($value, $MIN_DEFAULT_BYTES, $MAX_TODO_NAME_BYTES);
DEFINE INDEX IF NOT EXISTS idx_priority_name_unique ON TABLE priority COLUMNS name UNIQUE;


-- ToDo Tasks to Priority relation
DEFINE TABLE IF NOT EXISTS todo_task_priority SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS task_id ON TABLE todo_task_priority TYPE record<todo_tasks>;
DEFINE FIELD IF NOT EXISTS priority_id ON TABLE todo_task_priority TYPE record<priority>;

DEFINE INDEX IF NOT EXISTS idx_todo_task_priority_task ON TABLE todo_task_priority COLUMNS task_id;
DEFINE INDEX IF NOT EXISTS idx_todo_task_priority_priority ON TABLE todo_task_priority COLUMNS priority_id;


-- Calendar table
DEFINE TABLE IF NOT EXISTS calendar_events SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS board_id ON TABLE calendar_events TYPE record<project_board>;
DEFINE FIELD IF NOT EXISTS title ON TABLE calendar_events TYPE string ASSERT fn::bytes_within($value, $MIN_DEFAULT_BYTES, $MAX_CALENDAR_TITLE_BYTES);
DEFINE FIELD IF NOT EXISTS description ON TABLE calendar_events TYPE option<string> ASSERT $value = NONE OR fn::bytes_within($value, $MIN_NOTHING_BYTES, $MAX_CALENDAR_DESCRIPTION_BYTES);
DEFINE FIELD IF NOT EXISTS start_datetime ON TABLE calendar_events TYPE datetime;
DEFINE FIELD IF NOT EXISTS end_datetime ON TABLE calendar_events TYPE datetime ASSERT $value > start_datetime;
DEFINE FIELD IF NOT EXISTS task_id ON TABLE calendar_events TYPE option<record<todo_tasks>>;
DEFINE FIELD IF NOT EXISTS created_at ON TABLE calendar_events TYPE datetime;
DEFINE FIELD IF NOT EXISTS updated_at ON TABLE calendar_events TYPE option<datetime>;
DEFINE FIELD IF NOT EXISTS created_by ON TABLE calendar_events TYPE record<users>;

DEFINE INDEX IF NOT EXISTS idx_calendar_events_board ON TABLE calendar_events COLUMNS board_id;
DEFINE INDEX IF NOT EXISTS idx_calendar_events_date_range ON TABLE calendar_events COLUMNS board_id, start_datetime, end_datetime;
DEFINE INDEX IF NOT EXISTS idx_calendar_events_task ON TABLE calendar_events COLUMNS task_id;


-- Kanban columns
DEFINE TABLE IF NOT EXISTS kanban_columns SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS board_id ON TABLE kanban_columns TYPE record<project_board>;
DEFINE FIELD IF NOT EXISTS name ON TABLE kanban_columns TYPE string ASSERT fn::bytes_within($value, $MIN_DEFAULT_BYTES, $MAX_KANBAN_COLUMN_NAME_BYTES);
DEFINE FIELD IF NOT EXISTS position ON TABLE kanban_columns TYPE int ASSERT $value >= 0;

DEFINE INDEX IF NOT EXISTS idx_kanban_columns_board ON TABLE kanban_columns COLUMNS board_id;
DEFINE INDEX IF NOT EXISTS idx_kanban_columns_board_position ON TABLE kanban_columns COLUMNS board_id, position;


-- Kanban tasks
DEFINE TABLE IF NOT EXISTS kanban_tasks SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS kanban_column_id ON TABLE kanban_tasks TYPE record<kanban_columns>;
DEFINE FIELD IF NOT EXISTS title ON TABLE kanban_tasks TYPE string ASSERT fn::bytes_within($value, $MIN_DEFAULT_BYTES, $MAX_KANBAN_TASK_TITLE_BYTES);
DEFINE FIELD IF NOT EXISTS description ON TABLE kanban_tasks TYPE option<string> ASSERT $value = NONE OR fn::bytes_within($value, $MIN_NOTHING_BYTES, $MAX_KANBAN_TASK_DESCRIPTION_BYTES);
DEFINE FIELD IF NOT EXISTS priority_id ON TABLE kanban_tasks TYPE option<record<priority>>;
DEFINE FIELD IF NOT EXISTS created_at ON TABLE kanban_tasks TYPE datetime;
DEFINE FIELD IF NOT EXISTS updated_at ON TABLE kanban_tasks TYPE option<datetime>;

DEFINE INDEX IF NOT EXISTS idx_kanban_tasks_column ON TABLE kanban_tasks COLUMNS kanban_column_id;


-- Kanban task assignees
DEFINE TABLE IF NOT EXISTS kanban_task_assignees SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS task_id ON TABLE kanban_task_assignees TYPE record<kanban_tasks>;
DEFINE FIELD IF NOT EXISTS user_id ON TABLE kanban_task_assignees TYPE record<users>;

DEFINE INDEX IF NOT EXISTS idx_kanban_task_assignees_task ON TABLE kanban_task_assignees COLUMNS task_id;
DEFINE INDEX IF NOT EXISTS idx_kanban_task_assignees_user ON TABLE kanban_task_assignees COLUMNS user_id;
DEFINE INDEX IF NOT EXISTS idx_kanban_task_assignees_task_user_unique ON TABLE kanban_task_assignees COLUMNS task_id, user_id UNIQUE;