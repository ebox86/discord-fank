<template>
    <va-navbar color="black" text-color="#cdcdcd">
        <template #left>
        </template>
        <template #right>
            <Auth />
        </template>
    </va-navbar>
</template>

<script>
export default {
  name: 'AppHeader',
}
</script>

<style>
    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        background-color: #000;
        color: #fff;
    }
    .header ul {
        display: flex;
        list-style: none;
        padding-right: 5rem;
    }
    .header a {
        color: #ccc;
        display: inline-block;
        padding: 0.6rem 1rem;
        margin-right: 0.5rem;
    }
    </style>