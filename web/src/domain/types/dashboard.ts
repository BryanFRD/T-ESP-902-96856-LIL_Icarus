export type UserTeamMetrics = {
  total: number
  asOwner: number
  asMember: number
}

export type UserAuditMetrics = {
  asTeamMember: number
  asClient: number
  active: number
  archived: number
}

export type TicketsByPriority = {
  critical: number
  high: number
  medium: number
  low: number
}

export type UserTicketMetrics = {
  assigned: number
  created: number
  overdue: number
  byPriority: TicketsByPriority
  byPriorityAssigned: TicketsByPriority
}

export type ActivityMetrics = {
  total: number
  last7Days: number
  last30Days: number
}

export type UserDashboardMetrics = {
  teams: UserTeamMetrics
  audits: UserAuditMetrics
  tickets: UserTicketMetrics
  messages: ActivityMetrics
  comments: ActivityMetrics
  events: ActivityMetrics
}
