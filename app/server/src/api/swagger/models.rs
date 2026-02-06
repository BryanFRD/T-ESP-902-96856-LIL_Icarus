use crate::api::audits;
use crate::api::audits::invitations::{
    AcceptAuditInvitationPayload, AcceptInvitationResponse, AuditInvitationReturn,
    CheckInvitationResponse, CreateAuditInvitationPayload,
};
use crate::api::audits::step_up::endpoints::{CheckAccessResponse, VerifyAccessPayload};
use crate::api::audits::tickets;
use crate::api::audits::tickets::attachments::models::AttachmentResponse;
use crate::api::audits::tickets::columns::models::{
    ColumnResponse, CreateColumnPayload, ReorderColumnsPayload, UpdateColumnPayload,
};
use crate::api::audits::tickets::comments::models::{
    CommentResponse, CreateCommentPayload, UpdateCommentPayload,
};
use crate::api::audits::tickets::labels::models::{
    CreateLabelPayload, LabelResponse, UpdateLabelPayload,
};
use crate::api::audits::tickets::models::{
    BoardColumn, BoardResponse, CreateTicketPayload, MoveTicketPayload, TicketPriority,
    TicketResponse, TicketSummary, TicketUser, UpdateTicketPayload,
};
use crate::api::audits::{AuditResponse, ClientUser, CreateAuditPayload, UpdateAuditPayload};
use crate::api::auth;
use crate::api::auth::{
    ConfirmEmailPayload, LoginPayload, RegisterPayload, ResetPasswordPayload,
    ResetPasswordRequestPayload,
};
use crate::api::csrf;
use crate::api::csrf::CsrfResponse;
use crate::api::events;
use crate::api::events::{
    Event, EventAction, EventChanges, EventFilters, EventResponse, ResourceType,
};
use crate::api::status;
use crate::api::status::{Status, StatusEnum};
use crate::api::teams;
use crate::api::teams::models::UpdateMemberRolePayload;
use crate::api::teams::{
    CompanyReturn, CreateCompanyPayload, CreateInvitationPayload, CreateTeam, InvitationReturn,
    TeamMemberWithRole, TeamReturn, TeamRole, UpdateCompanyPayload, UpdateTeam,
};
use crate::api::users;
use crate::api::users::{AvatarForm, UpdatePayload, User, UserRole};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        audits::endpoints::list_my_audits,
        audits::endpoints::create_audit,
        audits::endpoints::update_audit,
        audits::endpoints::archive_audit,
        audits::endpoints::remove_client,
        audits::step_up::endpoints::verify_access,
        audits::step_up::endpoints::check_access,
        audits::invitations::endpoints::create_invitation,
        audits::invitations::endpoints::list_invitations,
        audits::invitations::endpoints::cancel_invitation,
        audits::invitations::endpoints::check_invitation,
        audits::invitations::endpoints::accept_invitation,
        // Kanban Board
        tickets::endpoints::get_board,
        // Kanban Columns
        tickets::columns::endpoints::list_columns,
        tickets::columns::endpoints::create_column,
        tickets::columns::endpoints::update_column,
        tickets::columns::endpoints::delete_column,
        tickets::columns::endpoints::reorder_columns,
        // Tickets
        tickets::endpoints::list_tickets,
        tickets::endpoints::create_ticket,
        tickets::endpoints::get_ticket,
        tickets::endpoints::update_ticket,
        tickets::endpoints::delete_ticket,
        tickets::endpoints::move_ticket,
        tickets::endpoints::list_subtasks,
        // Labels
        tickets::labels::endpoints::list_labels,
        tickets::labels::endpoints::create_label,
        tickets::labels::endpoints::update_label,
        tickets::labels::endpoints::delete_label,
        tickets::labels::endpoints::attach_label,
        tickets::labels::endpoints::detach_label,
        // Comments
        tickets::comments::endpoints::list_comments,
        tickets::comments::endpoints::create_comment,
        tickets::comments::endpoints::update_comment,
        tickets::comments::endpoints::delete_comment,
        // Attachments
        tickets::attachments::endpoints::list_attachments,
        tickets::attachments::endpoints::upload_attachment,
        tickets::attachments::endpoints::download_attachment,
        tickets::attachments::endpoints::delete_attachment,
        // Auth
        auth::endpoints::login,
        auth::endpoints::register,
        auth::endpoints::confirm_email,
        auth::endpoints::logout,
        csrf::get_csrf,
        auth::endpoints::request_password_reset,
        auth::endpoints::reset_password,
        events::endpoints::list_events,
        events::endpoints::get_event,
        users::endpoints::me,
        users::endpoints::update_me,
        users::endpoints::delete_me,
        users::endpoints::list_users_admin,
        users::endpoints::get_user_admin,
        users::endpoints::update_user_admin,
        users::endpoints::delete_user_admin,
        users::endpoints::upload_avatar,
        users::endpoints::delete_avatar,
        users::endpoints::serve_avatar,
        teams::endpoints::create_team,
        teams::endpoints::get_my_teams,
        teams::endpoints::update_team,
        teams::endpoints::delete_team,
        teams::endpoints::remove_member,
        teams::endpoints::update_member_role,
        teams::invitations::endpoints::create_invitation,
        teams::invitations::endpoints::list_invitations,
        teams::invitations::endpoints::delete_invitation,
        teams::invitations::endpoints::consume_invitation,
        teams::companies::endpoints::create_company,
        teams::companies::endpoints::list_companies,
        teams::companies::endpoints::get_company,
        teams::companies::endpoints::update_company,
        teams::companies::endpoints::delete_company,
        status::endpoints::get_status,
    ),
    components(
        schemas(
            CreateAuditPayload, UpdateAuditPayload, AuditResponse, ClientUser, VerifyAccessPayload, CheckAccessResponse,
            CreateAuditInvitationPayload, AcceptAuditInvitationPayload, AcceptInvitationResponse, AuditInvitationReturn, CheckInvitationResponse,
            ConfirmEmailPayload, LoginPayload, RegisterPayload, ResetPasswordRequestPayload, ResetPasswordPayload,
            CreateTeam, UpdateTeam, TeamReturn, CreateInvitationPayload, InvitationReturn, UpdateMemberRolePayload, TeamRole, TeamMemberWithRole,
            CompanyReturn, CreateCompanyPayload, UpdateCompanyPayload,
            User, UserRole, UpdatePayload, AvatarForm,
            Event, EventAction, EventChanges, EventFilters, EventResponse, ResourceType,
            Status, StatusEnum, CsrfResponse,
            ColumnResponse, CreateColumnPayload, UpdateColumnPayload, ReorderColumnsPayload,
            CreateTicketPayload, UpdateTicketPayload, MoveTicketPayload, TicketPriority, TicketResponse, TicketSummary, TicketUser, BoardColumn, BoardResponse,
            LabelResponse, CreateLabelPayload, UpdateLabelPayload,
            CommentResponse, CreateCommentPayload, UpdateCommentPayload,
            AttachmentResponse,
        ),
    ),
    tags(
        (name = "Auth", description = "Authentication endpoints"),
        (name = "Events", description = "Event logging and audit endpoints"),
        (name = "User", description = "User management endpoints"),
        (name = "Admin", description = "Admin management endpoints"),
        (name = "Status", description = "Application status endpoints"),
        (name = "Teams", description = "Team management endpoints"),
        (name = "Audits", description = "Audit management endpoints"),
        (name = "Tickets", description = "Kanban ticket management endpoints"),
        (name = "Kanban Columns", description = "Kanban column management endpoints"),
        (name = "Ticket Labels", description = "Ticket label management endpoints"),
        (name = "Ticket Comments", description = "Ticket comment management endpoints"),
        (name = "Ticket Attachments", description = "Ticket attachment management endpoints"),
    ),
    security(
        ("cookieAuth" = [])
    ),
)]
pub struct ApiDoc;
