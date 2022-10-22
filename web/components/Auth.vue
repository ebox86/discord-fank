<script setup>
import {discordLogin, useDiscordUser, useDiscordGuilds} from "../composables/discord";
const login = discordLogin
const user = await useDiscordUser()
// const guilds = await useDiscordGuilds()
</script>

<template>
    <div v-if="user" class="header">
        <va-navbar-item><nuxt-link to="/dashboard">Dashboard</nuxt-link></va-navbar-item>
        <va-navbar-item><nuxt-link to="/rank">Rank</nuxt-link></va-navbar-item>
    </div>
    <User v-if="user" :user="user" />
    <va-button color="#9f03e0" v-else @click="login">Login</va-button>
    <!-- <Guild :guilds="guilds" /> -->
</template>

