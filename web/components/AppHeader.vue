<template>
    <header class="header">
        <div class="header__logo">
            <img src="~/assets/logo.png" alt="Fankbot Logo" />
        </div>
        <ul>
            <li>
                <nuxt-link to="/">Home</nuxt-link>
            </li>
            <li>
                <nuxt-link to="/rank">Rank</nuxt-link>
            </li>
        </ul>
    </header>
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
        margin-bottom: 1rem;
        padding-bottom: 1rem;
        border-bottom: 5px solid rgb(255, 0, 0);
        background-color: #000;
        color: #fff;
        padding: 10px;
    }
    .header .title {
        font-size: 3rem;
        font-weight: bold;
    }

    .header ul {
        display: flex;
        list-style: none;
        margin: 0;
        padding: 0;
    }

    .header a {
        color: #ccc;
        display: inline-block;
        padding: 0.3rem 1rem;
        margin-right: 0.5rem;
    }
</style>