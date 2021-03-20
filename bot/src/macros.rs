#[macro_export]
macro_rules! send {
    (($ctx:expr, $msg:expr) => $($arg:tt)*) => {{
        let text = ::std::fmt::format(::std::format_args!($($arg)*));
        let res = $msg.channel_id
            .say(&$ctx.http, text)
            .await;
        if let Err(e) = res {
            error!("Failed to send message {:?}", e);
        }
    }}
}

#[macro_export]
macro_rules! send_to_channel {
    (($ctx:expr, $channel:expr) => $($arg:tt)*) => {{
        let text = ::std::fmt::format(::std::format_args!($($arg)*));
        let res = $channel.id
            .send_message(&$ctx, |c| c.content(text))
            .await;
        if let Err(e) = res {
            error!("Failed to send message {:?}", e);
        }
    }}
}

#[macro_export]
macro_rules! reply {
    (($ctx:expr, $msg:expr) => $($arg:tt)*) => {{
        let text = ::std::fmt::format(::std::format_args!($($arg)*));
        let res = $msg
            .reply(&$ctx.http, text)
            .await;
        if let Err(e) = res {
            error!("Failed to send message {:?}", e);
        }
    }}
}

#[macro_export]
macro_rules! reply_with_ping {
    (($ctx:expr, $msg:expr) => $($arg:tt)*) => {{
        let text = ::std::fmt::format(::std::format_args!($($arg)*));
        let res = $msg
            .reply_ping(&$ctx.http, text)
            .await;
        if let Err(e) = res {
            error!("Failed to send message {:?}", e);
        }
    }}
}

#[macro_export]
macro_rules! set_redis {
    (($ctx:expr, $msg:expr, &guild_id:expr, $locale:expr) => $k:expr, $v:expr) => {{
        let data = $ctx.data.as_ref().read().await;
        let con = data.get::<RedisConnection>().unwrap().clone();
        let con = con.lock().await;
        let mut con = match con.get_async_connection().await {
            Ok(conn) => conn,
            Err(e) => {
                error!("Failed to get Redis connection; {:?}", e);
                reply!(($ctx, $msg) => "{}", tt(&$locale, "UnexpectedError"));
                return Ok(());
            }
        };
        let _: i64 = match con.set(
            &format!("bot:channel:joined:{}", $guild_id.0),
            $msg.channel_id.0,
        ).await {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to set redis; {:?}", e);
                return Ok(());
            }
        };
    }}
}
