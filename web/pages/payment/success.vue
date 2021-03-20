<template>
  <form id="manage-billing-form">
    <button @click="submit">Manage Billing</button>
  </form>
</template>

<script lang="ts">
import { Vue, Component } from 'nuxt-property-decorator'

@Component({})
class Success extends Vue {
  sessionId = ''

  async mounted() {
    const sessionId = this.$route.params.sessionId
    if (sessionId) {
      const { data } = await this.$axios.$get(
        `/pay/checkout-session?sessionId=${sessionId}`
      )
      const dom = document.querySelector('pre')
      if (dom) {
        dom.textContent = JSON.stringify(data, null, 2)
      }
    }
  }

  async submit() {
    const { data } = await this.$axios.$post(
      '/customer-portal',
      {
        sessionId: this.sessionId,
      },
      {
        headers: {
          'Content-Type': 'application/json',
        },
      }
    )
    window.location.href = data.url
  }
}
export default Success
</script>
