import { invoke } from '@tauri-apps/api/core'
import type {
  AppSettings,
  AssetsInfo,
  BootstrapState,
  BrowseResult,
  ComponentSpec,
  DiscoveredHost,
  EnvironmentInfo,
  Host,
  HostInput,
  InstalledComponent,
  LogEntry,
  MoonlightSettings,
  PairingResult,
  PendingPairing,
  ServiceStatus,
  SessionSnapshot,
  StreamProfile,
  TestHostResult,
  ThemeConfig,
  ThemePreset,
} from './types'

export * from './types'

export function errorMessage(error: unknown): string {
  if (typeof error === 'string') return error
  if (error && typeof error === 'object') {
    const candidate = error as { message?: string; code?: string; hint?: string }
    const parts = [candidate.message ?? JSON.stringify(error)]
    if (candidate.hint) parts.push(candidate.hint)
    return parts.join(' · ')
  }
  return String(error)
}

// Bootstrap / system
export const getBootstrap = () => invoke<BootstrapState>('get_bootstrap')
export const getEnvironment = () => invoke<EnvironmentInfo>('get_environment')
export const getAppLogs = (limit = 500) => invoke<LogEntry[]>('get_app_logs', { limit })
export const clearAppLogs = () => invoke<void>('clear_app_logs')
export const getLogPath = () => invoke<string | null>('get_log_path')
export const setAutostart = (enabled: boolean) => invoke<void>('set_autostart', { enabled })
export const getAutostart = () => invoke<boolean>('get_autostart')
export const setGlobalHotkey = (accelerator: string) => invoke<string>('set_global_hotkey', { accelerator })
export const showMainWindow = () => invoke<void>('show_main_window')
export const setWindowEffects = (acrylic: boolean, dark: boolean) =>
  invoke<void>('set_window_effects', { acrylic, dark })
export const openMainWindow = () => invoke<void>('open_main_window')
export const quitApp = () => invoke<void>('quit_app')
export const toggleHud = () => invoke<void>('toggle_hud')
export const showHud = () => invoke<void>('show_hud')
export const exportDiagnostics = () => invoke<string>('export_diagnostics')
export const saveAppSettings = (settings: AppSettings) => invoke<AppSettings>('save_app_settings', { settings })
export const completeOnboarding = () => invoke<void>('complete_onboarding')

// Hosts
export const listHosts = () => invoke<Host[]>('list_hosts')
export const addHost = (input: HostInput) => invoke<Host>('add_host', { input })
export const updateHost = (input: HostInput) => invoke<Host>('update_host', { input })
export const removeHost = (id: string) => invoke<void>('remove_host', { id })
export const testHost = (input: HostInput) => invoke<TestHostResult>('test_host', { input })
export const connectHost = (id: string) => invoke<Record<string, unknown>>('connect_host', { id })
export const discoverHosts = (timeoutMs = 3000) => invoke<DiscoveredHost[]>('discover_hosts', { timeoutMs })
export const addDiscoveredHost = (discovered: DiscoveredHost) => invoke<Host>('add_discovered_host', { discovered })
export const setActiveHost = (id: string | null) => invoke<void>('set_active_host', { id })
export const resetHostFingerprint = (id: string) => invoke<Host>('reset_host_fingerprint', { id })

// Sunshine
export const getHostConfig = (hostId: string) => invoke<Record<string, unknown>>('get_config', { hostId })
export const getSessionStatus = (hostId: string) =>
  invoke<Record<string, unknown>>('get_session_status', { hostId })
export const saveHostConfig = (hostId: string, config: Record<string, unknown>) =>
  invoke<Record<string, unknown>>('save_config', { hostId, config })
export const getConfigLocale = (hostId: string) => invoke<Record<string, unknown>>('get_config_locale', { hostId })
export const getHostApps = (hostId: string) => invoke<unknown>('get_apps', { hostId })
export const saveHostApps = (hostId: string, apps: unknown) => invoke<unknown>('save_apps', { hostId, apps })
export const deleteHostApp = (hostId: string, index: number) => invoke<unknown>('delete_app', { hostId, index })
export const closeHostApp = (hostId: string) => invoke<unknown>('close_app', { hostId })
export const getHostClients = (hostId: string) => invoke<unknown>('get_clients', { hostId })
export const unpairClient = (hostId: string, uuid: string) => invoke<unknown>('unpair_client', { hostId, uuid })
export const unpairAll = (hostId: string) => invoke<unknown>('unpair_all', { hostId })
export const updateClient = (hostId: string, client: unknown) => invoke<unknown>('update_client', { hostId, client })
export const getPendingPairings = (hostId: string) => invoke<PendingPairing[]>('get_pending_pairings', { hostId })
export const submitPin = (hostId: string, pairingId: string, pin: string, name: string) =>
  invoke<unknown>('submit_pin', { hostId, pairingId, pin, name })
export const cancelPairing = (hostId: string, pairingId: string) =>
  invoke<unknown>('cancel_pairing', { hostId, pairingId })
export const changePassword = (hostId: string, payload: Record<string, unknown>) =>
  invoke<unknown>('change_password', { hostId, payload })
export const getHostLogs = (hostId: string) => invoke<string>('get_host_logs', { hostId })
export const restartHost = (hostId: string) => invoke<void>('restart_host', { hostId })
export const browseHost = (hostId: string, path = '', kind = 'any') =>
  invoke<BrowseResult>('browse_host', { hostId, path, kind })
export const getCover = (hostId: string, index: number) => invoke<string>('get_cover', { hostId, index })
export const getPermissions = (hostId: string) => invoke<Record<string, unknown>>('get_permissions', { hostId })
export const requestPermission = (hostId: string, id: string) =>
  invoke<Record<string, unknown>>('request_permission', { hostId, id })
export const getVirtualInputStatus = (hostId: string) =>
  invoke<Record<string, unknown>>('get_virtual_input_status', { hostId })
export const getVirtualInputLicense = (hostId: string) =>
  invoke<Record<string, unknown>>('get_virtual_input_license', { hostId })
export const resetDisplayPersistence = (hostId: string) =>
  invoke<Record<string, unknown>>('reset_display_persistence', { hostId })
export const getCsrfToken = (hostId: string) => invoke<Record<string, unknown>>('get_csrf_token', { hostId })
export const pairWithPin = (hostId: string, pin: string, name?: string) =>
  invoke<PairingResult>('pair_with_pin', { hostId, pin, name })

// Components & services
export const listComponents = () => invoke<InstalledComponent[]>('list_components')
export const getComponentManifest = () => invoke<ComponentSpec[]>('get_component_manifest')
export const installComponent = (id: string) => invoke<InstalledComponent>('install_component', { id })
export const removeComponent = (id: string) => invoke<void>('remove_component', { id })
export const getServiceStatus = () => invoke<ServiceStatus>('get_service_status')
export const startSunshineService = () => invoke<ServiceStatus>('start_sunshine_service')
export const stopSunshineService = () => invoke<ServiceStatus>('stop_sunshine_service')
export const installSunshineService = () => invoke<ServiceStatus>('install_sunshine_service')
export const uninstallSunshineService = () => invoke<ServiceStatus>('uninstall_sunshine_service')
export const addFirewallRules = () => invoke<void>('add_firewall_rules')
export const setSunshineCredentials = (username: string, password: string) =>
  invoke<void>('set_sunshine_credentials', { username, password })
export const detectComponents = () => invoke<EnvironmentInfo>('detect_components')
export const setComponentPath = (id: string, path: string | null) =>
  invoke<void>('set_component_path', { id, path })
export const validateComponentPath = (id: string, path: string) =>
  invoke<boolean>('validate_component_path', { id, path })

// Moonlight
export const getMoonlightSettings = () => invoke<MoonlightSettings>('get_moonlight_settings')
export const saveMoonlightSettings = (sections: Record<string, Record<string, unknown>>) =>
  invoke<MoonlightSettings>('save_moonlight_settings', { sections })
export const getStreamProfile = () => invoke<StreamProfile>('get_stream_profile')
export const saveStreamProfile = (profile: StreamProfile) => invoke<StreamProfile>('save_stream_profile', { profile })
export const launchSession = (hostId: string, appName: string, profile?: StreamProfile) =>
  invoke<SessionSnapshot>('launch_session', { hostId, appName, profile })
export const stopSession = () => invoke<SessionSnapshot>('stop_session')
export const getSession = () => invoke<SessionSnapshot>('get_session')
export const quitRemote = (hostId: string) => invoke<void>('quit_remote', { hostId })

// Theme
export const getTheme = () => invoke<ThemeConfig>('get_theme')
export const listThemePresets = () => invoke<ThemePreset[]>('list_theme_presets')
export const saveTheme = (theme: ThemeConfig) => invoke<ThemeConfig>('save_theme', { theme })
export const exportTheme = () => invoke<string>('export_theme')
export const exportThemeFile = (path: string) => invoke<void>('export_theme_file', { path })
export const importTheme = (json: string) => invoke<ThemeConfig>('import_theme', { json })
export const importThemeFile = (path: string) => invoke<ThemeConfig>('import_theme_file', { path })
export const importAsset = (kind: 'font' | 'background', path: string) =>
  invoke<string>('import_asset', { kind, path })
export const listAssets = () => invoke<AssetsInfo>('list_assets')
export const getAssetDataUrl = (relative: string) => invoke<string>('get_asset_data_url', { relative })
export const getRawTheme = () => invoke<Record<string, unknown>>('get_raw_theme')
