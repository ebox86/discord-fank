<template>
    <div class="container">
        <AppHeader />
        <slot />
    </div>
</template>

<script>
import AppHeader from '../components/AppHeader.vue'

export default {
  components: {
    AppHeader
  },

}


</script>

<style>
.container {
  max-width: 800px;
  margin: 2rem auto;
  overflow: hidden;
  padding: 1rem 2rem;
  background: #fff;
}
</style>