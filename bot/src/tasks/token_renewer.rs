use crate::data::GcpAccessToken;
use serenity::prelude::*;
use std::{sync::Arc, time::Duration};

macro_rules! unwrap {
    ($e:expr) => {
        match $e {
            Some(e) => e,
            None => continue,
        }
    }
}

pub async fn renew_token(ctx: Arc<Context>) {
    let ctx = Arc::clone(&ctx);
    tokio::spawn(async move {
        loop {
            info!("Renewing access token");
            let data = ctx.data.read().await;
            let token = unwrap!(data.get::<GcpAccessToken>());
            for _ in 0..5i32 {
                let mut token = token.lock().await;
                if token.renew_token().await.is_err() {
                    warn!("Renew token failed");
                    tokio::time::delay_for(Duration::from_secs(3)).await;
                } else {
                    info!("New Token is: {:?}", token.show());
                    break
                }
                error!("Renew token failed for 5 times")
            }
            tokio::time::delay_for(Duration::from_secs(1000)).await;
        }
    });
}

