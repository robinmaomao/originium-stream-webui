import { createRouter, createWebHashHistory } from 'vue-router'

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', redirect: '/local' },
    { path: '/onboarding', name: 'onboarding', component: () => import('@/views/OnboardingView.vue') },
    { path: '/local', name: 'local', component: () => import('@/views/LocalHostView.vue') },
    { path: '/hosts', name: 'hosts', component: () => import('@/views/HostsView.vue') },
    {
      path: '/host/:id',
      name: 'host-detail',
      component: () => import('@/views/HostDetailView.vue'),
      props: true,
    },
    { path: '/session', name: 'session', component: () => import('@/views/SessionView.vue') },
    { path: '/settings', name: 'settings', component: () => import('@/views/SettingsView.vue') },
    { path: '/logs', name: 'logs', component: () => import('@/views/LogsView.vue') },
    { path: '/hud', name: 'hud', component: () => import('@/views/SessionHud.vue') },
  ],
})

export default router
