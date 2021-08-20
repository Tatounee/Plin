# Plin
Tools for Clash Royale

# How to install it

## Token

You will need to get the token of your discord bot and the one of your Clash Royal API key.

- [Discord](https://discord.com/developers)
- [Clash Royal](https://developer.clashroyale.com/#/)

If you don't know how it's work, google it. :smirk:

## Setup

- Download the bot [here](https://github.com/Tatounee/Plin/releases/tag/v1.0) (*plin.exe*) or compile the [plin]() bin with [cargo](https://www.rust-lang.org/) (:exclamation: in release :exclamation:).
- Create a `.env` file next to it.
```
SomeFolder
  ├── plin.exe
  └── .env
```
- Setup your `.env` file
```
PLIN_DISCORD_TOKEN = <token of your discord bot>
PLIN_CR_TOKEN = <token of your Clash Royal api key>
```

## Run
Execute `plin.exe`. The programme will automatically create a folder for the database next to itself.
If after a certain amount of time the programme display an error that say that your key for the Clash Royal API is not valid cause the IP address registered don't match with your, just make a new key with your new IP address. 