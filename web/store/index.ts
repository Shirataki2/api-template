import { Store, ActionTree } from 'vuex'
import CookieParser from 'cookie'
import Cookies from 'js-cookie'
import { initialiseStores } from '@/utils/store-accessor'
const JSONbig = require('json-bigint')
// eslint-disable-next-line @typescript-eslint/no-explicit-any
const initializer = (store: Store<any>): void => initialiseStores(store)
export const plugins = [initializer]

export interface RootState {}

export const actions: ActionTree<RootState, RootState> = {
  async nuxtServerInit({ getters, dispatch }, { req, $axios }) {
    if (req.headers.cookie) {
      const cookies = CookieParser.parse(req.headers.cookie)
      const actixSession = cookies['actix-session']
      if (actixSession) {
        dispatch('auth/login', actixSession)
        const user = getters['auth/user']
        if (!user) {
          try {
            const { data } = await $axios.get('/api/users/@me', {
              transformResponse: [(data: any) => JSONbig.parse(data)],
            })
            dispatch('auth/setUser', data)
          } catch {
            dispatch('auth/logout')
            dispatch('auth/setUser', null)
          }
        }
      }
    }
  },
  async nuxtClientInit({ getters, dispatch }, { $axios }) {
    const actixSession = getters['auth/actixSession']
    if (actixSession) {
      Cookies.set('actix_session', actixSession, {
        sameSite: 'Lax',
        secure: false,
      })
      const user = getters['auth/user']
      if (!user) {
        try {
          const { data } = await $axios.get('/api/users/@me', {
            transformResponse: [(data: any) => JSONbig.parse(data)],
          })
          dispatch('auth/setUser', data)
        } catch {
          dispatch('auth/logout')
          dispatch('auth/setUser', null)
        }
      }
    } else {
      Cookies.remove('actix_session')
    }
  },
}

export * from '@/utils/store-accessor'
