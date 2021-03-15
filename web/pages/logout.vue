<template>
  <v-row justify="center" align="center">
    <v-col cols="12" sm="8" md="6">
      <h1>Redirecting</h1>
    </v-col>
  </v-row>
</template>

<script lang="ts">
import { Vue, Component } from 'nuxt-property-decorator'

@Component({})
class Logout extends Vue {
  mounted() {
    const apiEndpoint = process.env.API_ENDPOINT
    location.href = `${apiEndpoint}/auth/logout`
  }
}
export default Logout
</script>
