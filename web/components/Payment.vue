<template>
  <v-card class="product">
    <v-card-text>
      <div>
        <p class="title" v-text="title" />
      </div>
      <div class="mt-9">
        <span class="yen">￥</span>
        <span class="price" v-text="product.price" />
      </div>
    </v-card-text>
    <v-card-actions>
      <v-btn color="success" text block @click="checkout">subscribe</v-btn>
    </v-card-actions>
  </v-card>
</template>

<script lang="ts">
import { Vue, Component, Prop } from 'nuxt-property-decorator'
import { Plan } from '@/models/payment'

@Component({})
class Payment extends Vue {
  @Prop({ required: true, type: Object })
  product!: Plan

  @Prop({ required: true, type: String })
  title!: string

  async checkout() {
    try {
      const data = await this.createCheckoutSession(this.product.key)
      console.log(data)
      await this.$stripe?.redirectToCheckout({
        sessionId: data.sessionId,
      })
    } catch (err) {
      console.error(err)
    }
  }

  async createCheckoutSession(priceId: string) {
    try {
      const data = await this.$axios.$post(
        '/pay/create-checkout-session',
        {},
        {
          params: { priceId },
        }
      )
      return data
    } catch (err) {}
  }
}
export default Payment
</script>

<style lang="scss" scoped>
.product {
  text-align: center;

  .title {
    font-size: 1.3em;
  }

  .yen {
    font-size: 1.8em;
  }

  .price {
    font-size: 4em;
    font-weight: 800;
  }
}
</style>
