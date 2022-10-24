# Transmission telegram rust (ttr) bot

Configurable bot to help your server

## Development

### Enable variables from .env

```bash
export $(cat .env | xargs)
```

## List of commands

Copy this to [@botfather](https://telegram.me/BotFather) when editing list of commands:

```text
version - Command to display current bot version
ping - Simple ping command
```

## Abilities

- [x] Ability to receive commands from telegram and show help

- [ ] Send command on startup

- [ ] Ability to receive messages and execute arbitrary code

- [ ] Ability to receive files after some dialog

- [ ] Receive commands via stdin or something like that from other programs (i.e. executed via cron)

## Dialogues state diagram

```mermaid
stateDiagram-v2
    state "Sends 'Hi' to all known users" as Startup
    state "Saves chat id" as Start
    state "Answers 'Pong'" as Pong
    state "Answers user data" as Me
    state "Answers bot's current version" as Version
    state "Sends 'Good bye' to all known users" as Shutdown

    [*] --> Startup: On startup
    Startup --> Idle

    Idle --> Start: /start
    Start --> Idle

    Idle --> Pong: /ping
    Pong --> Idle

    Idle --> Me: /me
    Me --> Idle

    Idle --> Version: /version
    Version --> Idle

    Idle --> Shutdown: Receives kill signal
    Shutdown --> [*]
```
