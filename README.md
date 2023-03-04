# rust-discord-bot
A very simple discord bot written in rust

## To use and install this Discord bot, you'll need to follow these steps:

1.) Install Rust: If you haven't already, install Rust on your machine by following the instructions on the Rust website (https://www.rust-lang.org/tools/install).<br>
2.) Create a new Discord bot: Go to the Discord Developer Portal (https://discord.com/developers/applications) and create a new bot. Copy the bot token and save it for later.<br>
3.) Invite the bot to your server: Go to the "OAuth2" section of the Discord Developer Portal and generate an invite link for your bot. Use the link to invite the bot to your server.<br>
4.) Clone the repository: Clone this repository to your local machine.<br>
5.) Set the bot token: Open the ".env" file in the root directory of the cloned repository and replace "YOUR_BOT_TOKEN_HERE" with your bot token.<br>
6.) Compile and run the bot: Open a terminal window in the root directory of the cloned repository and run the following command: cargo run --release<br>

Your rust discord bot should now be up and running, please invite it to your discord server, and ping it to test it. 
<br><br>

## Commands / Usage

Once the bot is running, you can use the commands in Discord by typing them into a channel where the bot is present. The available commands are:<br>

`!ping`: Responds with the bot's current response time in milliseconds.<br>
`!weather <city>`: Responds with the current weather for the specified city.<br>
`Custom commands`: You can add custom commands by editing the "commands.txt" file in the root directory of the repository. Each command should be on a separate line and should have the format <command>: <response>. 

For example, if you add the line "hello: Hi there!", then typing !hello in Discord will cause the bot to respond with "Hi there!".
