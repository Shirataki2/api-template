<template>
  <v-app dark>
    <AppHeader />
    <v-main>
      <v-container>
        <nuxt />
      </v-container>
    </v-main>
    <v-footer absolute app>
      <span>&copy; {{ new Date().getFullYear() }}</span>
    </v-footer>
  </v-app>
</template>

<script lang="ts">
import { Vue, Component } from 'nuxt-property-decorator'
import AppHeader from '@/components/AppHeader.vue'

@Component({
  components: { AppHeader },
})
class Default extends Vue {}
export default Default
</script>
