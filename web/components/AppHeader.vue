<template>
  <v-app-bar app collapse-on-scroll>
    <v-app-bar-nav-icon />
    <v-toolbar-title style="cursor: pointer" @click="$router.push('/')">
      <span id="app-title"> DisCalendar v2 </span>
    </v-toolbar-title>
    <v-spacer />
    <template v-if="isLoggedIn">
      <AccountMenu>
        <Avatar class="mr-5" :size="48" />
      </AccountMenu>
    </template>
    <template v-else>
      <v-btn text to="/login" nuxt>Login</v-btn>
    </template>
  </v-app-bar>
</template>

<script lang="ts">
import { Vue, Component } from 'nuxt-property-decorator'
import Avatar from '@/components/account/Avatar.vue'
import AccountMenu from '@/components/account/AccountMenu.vue'
import { auth } from '@/store'

@Component({
  components: { Avatar, AccountMenu },
})
class AppHeader extends Vue {
  get isLoggedIn(): boolean {
    return auth.loggedIn
  }
}
export default AppHeader
</script>

<style lang="scss" scoped>
@font-face {
  font-family: Logo;
  src: url(/UniSansHeavy.otf);
}

#app-title {
  font-family: Logo, sans-serif;
  font-size: 1.1em;
}
</style>
