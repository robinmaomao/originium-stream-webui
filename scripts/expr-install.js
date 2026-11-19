(async () => {
  try {
    const installed = await window.__TAURI_INTERNALS__.invoke('install_component', { id: 'moonlight' })
    const components = await window.__TAURI_INTERNALS__.invoke('list_components')
    return { ok: true, installed, components }
  } catch (error) {
    return { ok: false, error: String(error) }
  }
})()
