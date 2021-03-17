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
