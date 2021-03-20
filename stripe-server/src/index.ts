import express from 'express';
import path from 'path';

const app = express();

const envFilePath = path.resolve(__dirname, '../.env')
const env = require('dotenv').config({ path: envFilePath });
if (env.error) {
    throw new Error('Unable to load the .env file')
}

import Stripe from 'stripe'

const stripe = new Stripe(process.env.STRIPE_SECRET_KEY!, {
    apiVersion: '2020-08-27',
})

app.get('/checkout-session', async (req, res) => {
    const { sessionId } = req.query;
    if (typeof sessionId === 'string') {
        const session = await stripe.checkout.sessions.retrieve(sessionId);
        res.send(session);
    } else {
        res.status(422)
        res.send({
            error: {
                message: 'invalid payload',
            }
        })
    }
})

app.post('/create-checkout-session', async (req, res) => {
    const frontendEndpoint = process.env.FRONTEND_URL!
    const { priceId } = req.query;
    if (typeof priceId === 'string') {
        try {
            const session = await stripe.checkout.sessions.create({
                mode: 'subscription',
                payment_method_types: ['card'],
                line_items: [
                    {
                        price: priceId,
                        quantity: 1,
                    }
                ],
                success_url: `${frontendEndpoint}/payment/success?session_id={CHECKOUT_SESSION_ID}`,
                cancel_url: `${frontendEndpoint}/payment/cenceled`
            })
            res.send({
                sessionId: session.id,
            })
        } catch (e) {
            res.status(400)
            res.send({
                error: {
                    message: e.message,
                }
            })
        }
    } else {
        res.status(422)
        res.send({
            error: {
                message: 'invalid payload',
            }
        })
    }
})

const price = (key: string, prices: Stripe.Response<Stripe.ApiList<Stripe.Price>> ): { key: string; price: number; } => {
    return {
        key: key,
        price: prices.data.find(price => price.id === key)?.unit_amount || 0
    }
}

app.get('/setup', async (_req, res) => {
    const prices = await stripe.prices.list()
    res.send({
        basic1Month: price(process.env.BASIC_1_MONTH_KEY!, prices),
        basic3Month: price(process.env.BASIC_3_MONTH_KEY!, prices),
        basic6Month: price(process.env.BASIC_6_MONTH_KEY!, prices),
        basic1Year: price(process.env.BASIC_1_YEAR_KEY!, prices),
        pro1Month: price(process.env.PRO_1_MONTH_KEY!, prices),
        pro3Month: price(process.env.PRO_3_MONTH_KEY!, prices),
        pro6Month: price(process.env.PRO_6_MONTH_KEY!, prices),
        pro1Year: price(process.env.PRO_1_YEAR_KEY!, prices),
    })
})

app.post('/customer-portal', async (req, res) => {
    const { sessionId } = req.body
    if (typeof sessionId === 'string') {
        const checkoutSession = await stripe.checkout.sessions.retrieve(sessionId)
        const returnUrl = process.env.STRIPE_ENDPOINT!
        
        const portalSession = await stripe.billingPortal.sessions.create({
            customer: checkoutSession.customer as string,
            return_url: returnUrl
        })
        res.send({
            url: portalSession.url
        })
    }
})


const host = process.env.STRIPE_HOST!
const port = ~~process.env.STRIPE_PORT!
app.listen(port, host, () => console.log(`Listen at ${host}:${port}`))
