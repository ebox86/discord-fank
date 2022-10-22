<template>
    <div>
        <DashboardHeader />
        <slot/>
    </div>
</template>
<script>
import DashboardHeader from '../components/DashboardHeader.vue'

export default {
  components: {
    DashboardHeader
  },
}
</script>