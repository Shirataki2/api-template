import { Module, VuexModule, Mutation, Action } from 'vuex-module-decorators'
import { User } from '@/models/user'

@Module({
  name: 'auth',
  stateFactory: true,
  namespaced: true,
})
export default class AuthModule extends VuexModule {
  private _loggedIn: boolean = false
  private _actixSession: string | null = null
  private _user: User | null = null

  public get loggedIn() {
    return this._loggedIn
  }

  public get actixSession() {
    return this._actixSession
  }

  public get user() {
    return this._user
  }

  @Mutation
  private SET_LOGIN(state: boolean) {
    this._loggedIn = state
  }

  @Mutation
  private SET_SESSION(session: string | null) {
    this._actixSession = session
  }

  @Mutation
  private SET_USER(user: User | null) {
    this._user = user
  }

  @Action
  public login(actixSession: string | null) {
    this.SET_LOGIN(true)
    this.SET_SESSION(actixSession)
  }

  @Action
  public setUser(user: User | null) {
    this.SET_USER(user)
  }

  @Action
  public logout() {
    this.SET_LOGIN(false)
    this.SET_SESSION(null)
    this.SET_USER(null)
  }
}
