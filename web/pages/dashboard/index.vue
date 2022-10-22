<script setup>
import {useDiscordGuilds} from "../../composables/discord";
const guilds = await useDiscordGuilds()
definePageMeta({
  layout: "dashboard",
});

</script>

<template>
    <NuxtLayout>
        <div class="container">
            <div class="row my-4">
                <div class="flex-auto">
                    <h1 class="text-3xl">Dashboard</h1>
                    <p>Select a server that Fank is invited in to manage settings.</p>
                    <Guild :guilds="guilds"/>
                </div>
            </div>
        </div>
    </NuxtLayout>
</template>

<style>
.row {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    border: 1px dotted rgb(0, 0, 0);
    margin: 1rem 0;
}
.container {
    display: flex;
    align-items: center;
    width: 100%;
    padding-right: 15px;
    padding-left: 15px;
    margin-right: auto;
    margin-left: auto;
    
}
</style>