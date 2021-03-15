export function getAvatarUrl(userId: bigint, avatarId: string): string {
  return `https://cdn.discordapp.com/avatars/${userId}/${avatarId}.jpeg`
}
