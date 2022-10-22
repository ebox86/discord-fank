<script setup>
import {discordLogin} from "../composables/discord";
const login = discordLogin;
onMounted(() => {
    login();
});
</script>
<template>
</template>