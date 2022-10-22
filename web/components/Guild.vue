<script setup>
  defineProps({
    guilds: {
      type: Object,
      required: true,
    },
  });
</script>

<template>
    <div class="icon-view">
      <GuildIcon v-for="guild in guilds" :key="guild.id" :name="guild.name" :icon="guild.icon" :id="guild.id"/>
    </div>
</template>

<style>
    .icon-view {
        display: block;

        padding: 1rem;
        border: 1px dotted rgb(0, 0, 0);
        margin: 1rem 0;
    }
</style>