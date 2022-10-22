<template>
    <div class="guild-icon">
        <va-popover :message=name>
            <va-avatar v-if="icon" :src=icon size="large"/>
            <va-avatar v-else size="large" font-size="30px">{{ getFirstLetters(name) }}</va-avatar>
        </va-popover>
    </div>
</template>
    
<script>
export default {
    name: "GuildIcon",
    props: ['name', 'icon', 'id'],
    methods : {
    getFirstLetters(str) {
        const firstLetters = str
        .split(' ')
        .map(word => word[0])
        .join('')
        .toUpperCase();
    return firstLetters;
    }
    }
}
</script>

<style>
    .guild-icon {
        display: inline-block;
        padding: 1rem;
        border: 1px dotted rgb(0, 0, 0);
        margin: 1rem 0;
    }
</style>