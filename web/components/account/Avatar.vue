<template>
  <v-avatar :size="size">
    <img :src="avatarUrl" alt="User Avatar" />
  </v-avatar>
</template>

<script lang="ts">
import { Vue, Component, Prop } from 'nuxt-property-decorator'
import { auth } from '@/store'
import { getAvatarUrl } from '@/utils/discord'

@Component({})
class Avatar extends Vue {
  @Prop({ default: '48', type: Number })
  size!: number

  get avatarUrl(): string {
    if (auth.user) {
      return getAvatarUrl(auth.user.id, auth.user.avatar)
    } else {
      return ''
    }
  }
}
export default Avatar
</script>
