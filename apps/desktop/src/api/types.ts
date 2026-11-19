export type HostSource = 'manual' | 'mdns'

export interface Host {
  id: string
  name: string
  address: string
  port: number
  username?: string | null
  certFingerprint?: string | null
  source: HostSource
  paired: boolean
  favorite: boolean
  lastConnectedAt?: number | null
  note?: string | null
}

export interface HostInput {
  id?: string
  name: string
  address: string
  port: number
  username?: string
  password?: string
  source: HostSource
}

export interface StreamProfile {
  width: number
  height: number
  fps: number
  bitrateKbps: number
  videoCodec: string
  audioConfig: string
  displayMode: string
  hdr: boolean
  yuv444: boolean
  vsync: boolean
  framePacing: boolean
  quitAfter: boolean
  keepAwake: boolean
  performanceOverlay: boolean
  captureSystemKeys: string
  packetSize?: number | null
  extraArgs: string[]
}

export interface AppSettings {
  language: string
  autostart: boolean
  startMinimized: boolean
  closeToTray: boolean
  globalHotkey: string
  sunshinePath?: string | null
  moonlightPath?: string | null
  defaultProfile: StreamProfile
  updateChannel: string
  preferServiceMode: boolean
  autoStartServices: boolean
  acrylic: boolean
}

export interface ThemeColors {
  primary: string
  secondary: string
  surface: string
  background: string
  error: string
  success: string
  info: string
  warning: string
}

export interface ThemeTypography {
  fontFamily: string
  scale: number
}

export interface ThemeBackground {
  mode: string
  opacity: number
  blur: number
  image?: string | null
}

export interface ThemeEffects {
  rounded: number
  shadows: boolean
  motion: boolean
}

export interface ThemeConfig {
  version: number
  name: string
  preset: string
  colors: ThemeColors
  typography: ThemeTypography
  background: ThemeBackground
  effects: ThemeEffects
}

export interface AppConfig {
  schemaVersion: number
  onboardingComplete: boolean
  hosts: Host[]
  settings: AppSettings
  theme: ThemeConfig
  activeHost?: string | null
}

export interface ComponentSpec {
  id: string
  variant: string
  version: string
  upstream?: string | null
  url?: string | null
  sha256?: string | null
  entry: string
  capabilities: string[]
}

export interface InstalledComponent {
  id: string
  variant: string
  version: string
  path: string
  entry: string
  capabilities: string[]
  installedAt: number
}

export interface EnvironmentInfo {
  platform: string
  arch: string
  isAdmin: boolean
  sunshineInstalled: boolean
  sunshineService: string
  moonlightInstalled: boolean
  moonlightVersion?: string | null
  webview2Version?: string | null
  dataDir: string
  hostname: string
  lanIps: string[]
  appVersion: string
}

export type SessionState = 'idle' | 'launching' | 'streaming' | 'stopping' | 'error'

export interface SessionSnapshot {
  state: SessionState
  hostId?: string | null
  hostName?: string | null
  appName?: string | null
  startedAt?: number | null
  pid?: number | null
  error?: string | null
  logTail: string[]
}

export interface LogEntry {
  timestamp: string
  level: string
  target: string
  message: string
}

export interface BootstrapState {
  config: AppConfig
  components: InstalledComponent[]
  environment: EnvironmentInfo
  session: SessionSnapshot
}

export interface DiscoveredHost {
  name: string
  address: string
  port: number
  source: HostSource
}

export interface PendingPairing {
  id: string
  name: string
  address: string
}

export interface BrowseEntry {
  name: string
  path: string
  type: string
}

export interface BrowseResult {
  path: string
  parent: string
  entries: BrowseEntry[]
}

export interface TestHostResult {
  ok: boolean
  latencyMs: number
  platform: string
  version: string
  raw: Record<string, unknown>
}

export interface ServiceStatus {
  found: boolean
  name?: string | null
  state?: string | null
  startMode?: string | null
}

export interface MoonlightSettings {
  path: string
  exists: boolean
  sections: Record<string, Record<string, unknown>>
}

export interface AssetsInfo {
  fonts: string[]
  backgrounds: string[]
}

export interface ThemePreset {
  id: string
  name: string
  theme: ThemeConfig
}

export interface PairingResult {
  paired: boolean
  pairingId?: string | null
  message: string
}

export interface ComponentProgress {
  id: string
  phase: string
  percent: number
  message: string
}

export interface AppError {
  code: string
  message: string
  hint?: string | null
}
