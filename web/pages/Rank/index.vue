<template>
    <NuxtLayout>
        <div class="lists">
            <va-select
                class="mb-4"
                label="Guild Id"
                :options="options"
                v-model="options"
                loading
            />
            <va-data-table :items="ranks" v-model="value" :options="options" track-by="id" />
            <!-- <Rank v-for="rank in ranks" :key="rank.user_id" 
            :user_id="rank.user_id" :points="rank.points" :level="rank.level" :user_name="rank.user_name"/> -->
        </div>
    </NuxtLayout>
</template>

<script>
import Rank from "../../components/Rank.vue"

definePageMeta({
  layout: "dashboard",
});
export default {
    components: {
        Rank
    },
    data() {
    const options = [
      {
        text: '698169764861837372',
        value: '698169764861837372',
        id: '1',
      },
    ]
        return {
            ranks: [],
            value: options[0].value,
            options,
        }
    },
    async created(){
        const { BOT_URL } = useRuntimeConfig()
        const config = {
            headers: {
                Accept: "application/json",
            }
        }
        try {
            const response = await useFetch(`${BOT_URL}rank/show/${this.value}`)
            this.ranks = response.data;
        } catch (error) {
            console.log(error)
        }
    }
}

</script>

<style>
.lists {
    padding-top: 1rem;
    width: 100%;
    height: 100%;
}
</style>