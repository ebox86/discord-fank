<template>
    <va-navbar color="black" text-color="#cdcdcd">
        <template #left>
            <va-navbar-item>        
                <div>
                    <nuxt-link to="/">
                        <img class="logo" src="~/assets/logo.png" alt="Fankbot Logo" />
                    </nuxt-link>
                </div>
            </va-navbar-item>
        </template>
        <template #right>
            <Auth />
        </template>
    </va-navbar>
</template>

<script>
export default {
  name: 'DashboardHeader',
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
    .logo {
    height: auto; 
    width: auto;
    max-width: 100px; 
    }
</style>