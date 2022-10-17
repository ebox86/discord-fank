<template>

<div class="rank">
    <h2>user_name: {{ user_name }}</h2>
    <p>user_id: {{ user_id }}</p>
    <p>points: {{ points }}</p>
    <p>level: {{ level }}</p>
</div>

</template>

<script>
export default {
    name: "Rank",
    props: ['user_id', 'points', 'level', 'user_name']
}

</script>

<style>
    .rank {
        padding: 1rem;
        border: 1px dotted rgb(0, 0, 0);
        margin: 1rem 0;
    }
</style>