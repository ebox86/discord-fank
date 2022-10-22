<template>
    <div>
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