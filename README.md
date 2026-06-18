# Quorum - FHE
(Full Homomorphic Encryption)

> NOTE: AI was used as a **tool** here to help assist with the maths going on here as I am not at such a mathmatical level to create maths this complex.

---

For Quorum, the goals are privacy and security (amongst some others). Encryption for messages and other valuable user inputted data, was seen as a **must have**.
For this project, there was two choices to balance.
1) Don't include encryption (platforms such as discord as an example) don't use encryption. This allows them to moderate and have evidence of what goes on, but this also means it **doesn't respect user's privacy** which is against the goals of the project.
2) Include encryption (signal) - Keeps user's messages private and safe. Though the downside to this was user's abusing this privacy, being able to say anything they want (e.g., Whatsapp for example has lots of examples of the abuse that goes on because the messages can't be moderated or otherwise seen).

Both options above came with a heavy downside that was unpleasing and neither was really up for the task for this project. With a lot of research, [Homomorphic Encryption](https://en.wikipedia.org/wiki/Homomorphic_encryption) came up as a possible solution. 

> For anyone that doesn't understand this type of encryption, this is like any other encryption but allows computation on the encryption WITHOUT needing any keys or to decrypt it in anyway.

I developed this as a solution that satisfied what I needed for the project, keeping the privacy and security intaked whilst still being able to moderate user's messages.
The device that runs the moderation/scanning on encrypted text doesn't actually know what the message says and is never passed the key used to encrypt it. It uses maths to compare the banned words patterns with the encryption. The moderation only knows true or false when comparing banned words with the encryption (it only knows good message from bad message).

Currently, this only supports specific words, no AI systems or complex algorithms to decide whether a message has one or more flagged banned word in it. This may be improved on in the future if there is a need for something more complex.


## Contributing

Contributions are welcome. Please read [CONTRIBUTE.md](CONTRIBUTE.md) before opening a pull request.

---

## License

TBD — license will be added before public release.
