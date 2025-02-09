# Workshopper

Redistributable executable to interact with the Steam Workshop part of the SteamWorks API. This requires Steam to be running, and when used Steam will show you as playing the game you're using this exe for.

For compilation, you need a copy of the steam_api64.lib or steam_api64.dll (this one is only needed at runtime if you compiled this program without the .lib file). That file comes from the SteamWorks SDK.

# Usage

This is intended to be used as a companion app, not as a standalone app, due to the fact that Steam will register any app that initialize its API as "playing a game", and won't stop registering it until the application is closed. So every time you call this cli tool, it'll initialize with Steam, do something, then close, letting Steam know you've stopped "playing" said game. This allows your app to support multiple games whithin the same app without restarts. 

Communication for the commands that return data is done through named ipc sockets.

Also, some commands require the -b arg. This means the data is passed as a base64-encoded string. This is done because some strings may contain characters that break the cmd on windows.

Some usage examples:
```bash

# Get info about mods with workshop ids 2789857593, 2789857945 and 2789858755, for game with Steam ID 1142710 (Total War Warhammer 3), and return it through an IPC Socket with name "15288004326539387500". 
./workshopper.exe get-published-file-details -s 1142710 -p 2789857593,2789857945,2789858755 -i 15288004326539387500

# Force-download subscribed mods with workshop ids 2789857593, 2789857945 and 2789858755, for game with Steam ID 1142710 (Total War Warhammer 3). 
./workshopper.exe download-subscribed-items -s 1142710 -p 2789857593,2789857945,2789858755

# Upload a new mod to Total War Warhammer 3 workshop.
./workshopper.exe upload -b -s 1142710 -f path_to_the_mod_file -t titleencodedinbase64 --tags mod,units

# Update an existing mod to Total War Warhammer 3 workshop.
./workshopper.exe update --published-file-id idofthemodintheworkshop -b -s 1142710 -f path_to_the_mod_file -t titleencodedinbase64 --tags mod,units

```

# Limitations:

This comes with some limitations:

- **Single-File mods only**: while the workshop seems to supports uploading of folders and multiple files in one mod, for now this tool has been tweaked to keep compatibility with the Total War Launcher, which only supports uploading one file per mod. This limitation may be remove in the future.

- **Game/Workshop Support**: while this tools in theory supports any game with a Workshop, it has been mainly tested with Total War games. It uses the SteamWorks UGC API, which should be compatible with the deprecated SteamWorks Remote Storage API many games use, but there's no guarantee it'll work.

# Redistribution

For modders who want their tools to have any of the integrations this tool provides, you are free to redistribute this tool with your own tool.
