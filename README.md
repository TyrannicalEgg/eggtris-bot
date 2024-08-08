# Eggtris Battle Bot
This is a bot made for [Botris Battle](https://botrisbattle.com/)  
https://botrisbattle.com/


If you would like use this as a jumping off point for your own bot feel free.
I am going to continue working on it to hopefully make it better

## Bot State
Currently it is just a framework around the api provided by botris battle.
There is no logic and is currently missing many features I hope to add in the 
near future (aka. the bot part) 

## Usage
The connection can be setup with the following call:
``` rust
BotrisWebSocket::new().await;
```
This requires that the enviornment varibales TOKEN and ROOMKEY are set 
to their correspoding values which can be done in the .env file.

### Questions
If you have any questions, tips, or anything else feel free to contact me on 
discord.

<sup><sup>p.s. this is my first time ever writing anything in rust I saw this as
a good place to start so it may not be the best code ever  
also yes I know my commit messages are horrible :3</sup></sup>
