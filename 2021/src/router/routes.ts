import { RouteRecordRaw } from 'vue-router';

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: () => import('layouts/MainLayout.vue'),
    children: [{
      path: 'day-1',
      component: () => import('pages/Day1.vue'),
    }, {
      path: 'day-2',
      component: () => import('pages/Day2.vue'),
    }, {
      path: 'day-3',
      component: () => import('pages/Day3.vue'),
    }, {
      path: 'day-4',
      component: () => import('pages/Day4.vue'),
    }, {
      path: 'day-5',
      component: () => import('pages/Day5.vue'),
    }],
  },

  // Always leave this as last one,
  // but you can also remove it
  {
    path: '/:catchAll(.*)*',
    component: () => import('pages/Error404.vue'),
  },
];

export default routes;
