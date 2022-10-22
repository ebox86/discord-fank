<script setup>
defineProps({
  user: {
    type: Object,
    required: true,
  },
});
function logout(){
  const router = useRouter();
  discordLogout()
  router.push({ path: "/" });
}

const discord_avatar_url = "https://cdn.discordapp.com/avatars/"
</script>

<template>
    <va-avatar outline class="mr-3" size="2.25rem" :src="discord_avatar_url+user.id+'/'+user.avatar+'.png?size=64'" />
    <va-button color="#9f03e0" @click="logout">Logout</va-button>
</template>

