<template>
    <NuxtLayout>
        <div>
            <va-data-table :items="ranks" />
            <!-- <Rank v-for="rank in ranks" :key="rank.user_id" 
            :user_id="rank.user_id" :points="rank.points" :level="rank.level" :user_name="rank.user_name"/> -->
        </div>
    </NuxtLayout>
</template>

<script>
import Rank from "../../components/Rank.vue"

export default {
    components: {
        Rank
    },
    data() {
        return {
            ranks: []
        }
    },
    async created(){
        const config = {
            headers: {
                Accept: "application/json",
            }
        }
        try {
            const response = await useFetch("http://127.0.0.1:8000/rank/show")
            this.ranks = response.data;
        } catch (error) {
            console.log(error)
        }
    }
}

</script>