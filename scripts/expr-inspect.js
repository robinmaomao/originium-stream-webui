(() => {
  const app = document.getElementById('app')?.__vue_app__
  if (!app) return 'no app'
  return {
    hasVuetify: Boolean(app.config.globalProperties.$vuetify),
    provides: Object.keys(app._context.provides),
    components: Object.keys(app._context.components).length,
  }
})()
