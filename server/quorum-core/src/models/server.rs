#[derive(Default)]
pub struct AuditEvent {
    pub log_type: String,
    pub action: Option<String>,
    pub target_type_table: Option<String>,
    pub target_type_table_id: Option<String>,
    pub new_value: Option<String>,
    pub old_value: Option<String>,
    pub user_id: Option<String>,
}
