<template>
    <NuxtLayout>
        <div>
            <Rank v-for="rank in ranks" :key="rank.user_id" 
            :user_id="rank.user_id" :points="rank.points" :level="rank.level"/>
        </div>
    </NuxtLayout>
</template>

<script>
import axios from "axios"
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
            const response = await axios.get("http://localhost:8000/rank/show")
            this.ranks = response.data;
        } catch (error) {
            console.log(error)
        }
    }
}

</script>