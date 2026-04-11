export type SystemInfo = {
  hostname: string
  osName: string
  osVersion: string
  cpuName: string
  cpuCount: number
  totalMemoryBytes: number
  usedMemoryBytes: number
  uptimeSecs: number
  agentVersion: string
  gpuName: string | null
  username: string
  isElevated: boolean
  arch: string
  locale: string | null
  localIps: string[]
  macAddress: string | null
  domain: string | null
  totalDiskBytes: number
  usedDiskBytes: number
  antivirus: string[]
  firewallEnabled: boolean | null
}

export type Agent = {
  id: string
  auditId: string
  name: string
  isRevoked: boolean
  isOnline: boolean
  latencyMs: number | null
  lastConnectedAt: string | null
  lastHeartbeat: string | null
  systemInfo: SystemInfo | null
  serverVersion: string
  createdAt: string
}
