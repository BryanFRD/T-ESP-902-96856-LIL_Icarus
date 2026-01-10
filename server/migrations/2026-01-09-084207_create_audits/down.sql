-- Drop trigger and function
DROP TRIGGER IF EXISTS update_audits_updated_at ON audits;
DROP FUNCTION IF EXISTS update_audits_updated_at_func();

-- Drop indexes
DROP INDEX IF EXISTS idx_audits_clients_user_id;
DROP INDEX IF EXISTS idx_audits_clients_audit_id;
DROP INDEX IF EXISTS idx_audits_end_date;
DROP INDEX IF EXISTS idx_audits_is_archived;
DROP INDEX IF EXISTS idx_audits_company_id;
DROP INDEX IF EXISTS idx_audits_team_id;

-- Drop tables (audits_clients first due to foreign key)
DROP TABLE IF EXISTS audits_clients;
DROP TABLE IF EXISTS audits;
