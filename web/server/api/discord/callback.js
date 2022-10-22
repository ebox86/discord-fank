import { getQuery, sendRedirect, setCookie } from 'h3'

export default async (req, res) => {
  const code = getQuery(req).code.toString()
  console.log(code)
  if (!code) {
    console.log('No code provided')
    return sendRedirect(res, '/')
  }
  const { DISCORD_CLIENT_ID, DISCORD_CLIENT_SECRET, REDIRECT_URI } = useRuntimeConfig()
  console.log(`${REDIRECT_URI}api/discord/callback`)
  const response = await $fetch('https://discord.com/api/oauth2/token', {
  method: 'POST',
  body: new URLSearchParams({
    client_id: DISCORD_CLIENT_ID,
    client_secret: DISCORD_CLIENT_SECRET,
    code: code,
    grant_type: 'authorization_code',
    redirect_uri: `${REDIRECT_URI}api/discord/callback`,
    scope: 'identify guilds',
  }).toString(),
  headers: {
    'Content-Type': 'application/x-www-form-urlencoded',
  },
});

  if (response.error) {
    return sendRedirect(res, '/')
  }

  setCookie(res, 'discord_token', response.access_token, { path: '/' })

  return sendRedirect(res, '/dashboard')
}