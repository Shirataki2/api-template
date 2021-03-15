export interface User {
  avatar: string
  bot: boolean
  discriminator: number
  email?: string
  id: bigint
  username: string
  verified?: boolean
}
