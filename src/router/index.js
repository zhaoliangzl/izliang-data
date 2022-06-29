import Vue from 'vue'
import Router from 'vue-router'
import Index from '@/pages/Index'
import Settings from '@/pages/Settings'
import Mulit from '@/pages/Mulit'
Vue.use(Router)

export default new Router({
  routes: [
    {
      path: '/',
      name: 'Index',
      component: Index
    },
    {
      path: '/mulit',
      name: 'Mulit',
      component: Mulit
    },
    {
        path: '/settings',
        name: 'Settings',
        component: Settings
      },
    {
      path: '*',
      redirect: '/'
    }
  ]
})
