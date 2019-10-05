#[cfg(any(
    telegram_long_polling,
    telegram_webhook,
    telegram_raw_api,
    telegram_formed_api
))]
pub use rustbot_telegram as telegram;
