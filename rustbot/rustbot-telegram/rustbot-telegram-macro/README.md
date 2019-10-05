# rustbot-telegram-macro

Use like this, made for the easier telegram method mapping.

```rust
telegram_api_copied! {
sendContact -> Message

Use this method to send phone contacts. On success, the sent Message is returned.
Parameter 	Type 	Required 	Description
chat_id 	Integer or String 	Yes 	Unique identifier for the target chat or username of the target channel (in the format @channelusername)
phone_number 	String 	Yes 	Contact's phone number
first_name 	String 	Yes 	Contact's first name
last_name 	String 	Optional 	Contact's last name
vcard 	String 	Optional 	Additional data about the contact in the form of a vCard, 0-2048 bytes
disable_notification 	Boolean 	Optional 	Sends the message silently. Users will receive a notification with no sound.
reply_to_message_id 	Integer 	Optional 	If the message is a reply, ID of the original message
reply_markup 	InlineKeyboardMarkup or ReplyKeyboardMarkup or ReplyKeyboardRemove or ForceReply 	Optional 	Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove keyboard or to force a reply from the user.
}
```

will allow us to use below code.

```rust
let message: Message = sendContact()
  .chat_id(chat_id_value) // SendContact<ChatId: Completion = Complete, PhoneNumber: Completion = Incomplete, FirstName: Completion = Incomplete, FirstName: Completion,....>
  .phone_number(phone_number_value)
  .first_name(first_name_value)
  // belows are optional
  .last_name(last_name_value)
  .vcard(vcard_value)
  // specially, when the boolean field setter method called without argument, it will interpret as it was supplied true.
  .disable_notification()
  .reply_to_message(reply_to_message_value)
  .reply_markup(reply_markup_value)
  .invoke().await?;
```
