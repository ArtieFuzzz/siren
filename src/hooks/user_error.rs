use serenity::{
    client::Context,
    framework::standard::{macros::hook, DispatchError},
    model::channel::Message,
};

#[hook]
pub async fn delay_action(ctx: &Context, message: &Message) {
    let _ = message.react(ctx, '⏱').await;
}

#[hook]
pub async fn dispatch_error(ctx: &Context, message: &Message, error: DispatchError) {
    if let DispatchError::Ratelimited(ref info) = error {
        // We notify them only once.
        if info.is_first_try {
            let _ = message
                .channel_id
                .say(
                    &ctx.http,
                    &format!("Try this again in {} seconds.", info.as_secs()),
                )
                .await;
        }
    }

    if let DispatchError::NotEnoughArguments { min, given } = error {
        let _ = message
            .channel_id
            .say(
                &ctx.http,
                format!("{} Arguments must be given, {} was given", min, given),
            )
            .await;
    }
}
