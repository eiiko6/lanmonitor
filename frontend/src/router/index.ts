import { createRouter, createWebHistory } from 'vue-router';
import Home from '../pages/Home.vue';

const routes = [
  { path: '/', component: Home },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
  scrollBehavior(_to, _from, _savedPosition) {
    return { top: 0 };  // Always scroll to top on route change
  }
});

router.afterEach((to, from) => {
  const toDepth = to.path.split('/').length;
  const fromDepth = from.path.split('/').length;
  to.meta.transition = toDepth < fromDepth ? 'slide-right' : 'slide-left';
});

export default router;

