<template>
  <v-row justify="center" align="center">
    <v-col cols="12" md="6" lg="4">
      <v-row class="flex-column">
        <v-col cols="12">
          <p class="plan-header my-7">Basic Plan</p>
        </v-col>
        <v-col v-for="plan in basics" :key="plan.title" cols="12">
          <Payment :title="plan.title" :product="plan.plan" />
        </v-col>
      </v-row>
    </v-col>
    <v-col cols="12" md="6" lg="4">
      <v-row class="flex-column">
        <v-col cols="12">
          <p class="plan-header my-7">Pro Plan</p>
        </v-col>
        <v-col v-for="plan in pros" :key="plan.title" cols="12">
          <Payment :title="plan.title" :product="plan.plan" />
        </v-col>
      </v-row>
    </v-col>
  </v-row>
</template>

<script lang="ts">
import { Vue, Component } from 'nuxt-property-decorator'
import Payment from '@/components/Payment.vue'
import { Plan } from '@/models/payment'

@Component({
  components: { Payment },
  async asyncData({ $axios }) {
    const plans = await $axios.$get('/pay/setup')
    return { ...plans }
  },
})
class Plans extends Vue {
  basic1Month!: Plan
  basic3Month!: Plan
  basic6Month!: Plan
  basic1Year!: Plan
  pro1Month!: Plan
  pro3Month!: Plan
  pro6Month!: Plan
  pro1Year!: Plan

  get basics() {
    return [
      { title: '1ヶ月', plan: this.basic1Month },
      { title: '3ヶ月', plan: this.basic3Month },
      { title: '6ヶ月', plan: this.basic6Month },
      { title: '1年', plan: this.basic1Year },
    ]
  }

  get pros() {
    return [
      { title: '1ヶ月', plan: this.pro1Month },
      { title: '3ヶ月', plan: this.pro3Month },
      { title: '6ヶ月', plan: this.pro6Month },
      { title: '1年', plan: this.pro1Year },
    ]
  }
}
export default Plans
</script>

<style lang="scss" scoped>
.plan-header {
  text-align: center;
  font-size: 2em;
  font-weight: bold;
}
</style>
