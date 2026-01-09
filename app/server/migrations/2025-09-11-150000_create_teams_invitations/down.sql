DROP TRIGGER IF EXISTS trg_set_expires_at_on_cancel ON teams_invitations;
DROP FUNCTION IF EXISTS set_expires_at_on_cancel();
DROP TABLE IF EXISTS teams_invitations;

