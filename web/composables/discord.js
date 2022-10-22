export const useDiscordCookie = () => useCookie('discord_token')
import { convertPerms } from '../utils/permissions';

export const discordFetch = (url, fetchOptions = {}) => {
  return $fetch(url, {
    baseURL: 'https://discord.com/api',
    ...fetchOptions,
    headers: {
      Authorization: `Bearer ${useDiscordCookie().value}`,
      ...fetchOptions.headers,
    },
  })
}

export const useDiscordUser = async () => {
  const cookie = useDiscordCookie()
  const user = useState('discord_user')
  if (cookie.value && !user.value) {
    user.value = await discordFetch('/users/@me')
  }
  return user
}

export const useDiscordGuilds = async () => {
  const cookie = useDiscordCookie()
  const guilds = useState('discord_guilds')
  if (cookie.value && !guilds.value) {
    guilds.value = await discordFetch('/users/@me/guilds')
  }
  const guilds_with_perms = [];
  for (var i = 0; i < guilds.value.length; i++) {
    var perms = convertPerms(guilds.value[i].permissions)
    if(perms.ADMINISTRATOR || perms.MANAGE_GUILD) {
      let icon = guilds.value[i].icon ? `https://cdn.discordapp.com/icons/${guilds.value[i].id}/${guilds.value[i].icon}.png` : null
      guilds_with_perms.push({
        id: guilds.value[i].id,
        name: guilds.value[i].name,
        icon: icon,
        perms: JSON.stringify(perms)
      })
    }
  }
  return guilds_with_perms
}

export const discordLogin = () => {
  if (process.client) {
    const { DISCORD_CLIENT_ID } = useRuntimeConfig()
    window.location.replace(
      `https://discord.com/oauth2/authorize?client_id=${DISCORD_CLIENT_ID}&redirect_uri=http%3A%2F%2Flocalhost%3A3000%2Fapi%2Fdiscord%2Fcallback&response_type=code&scope=identify%20guilds`
    )
  }
}

export const discordLogout = async () => {
  useDiscordCookie().value = null
  useState('discord_user').value = null
  useState('discord_guilds').value = null
}