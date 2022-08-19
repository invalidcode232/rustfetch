# rustfetch
Just another system information tool written in Rust.  

Features:  
- Minimal
- Dependency-free
- Built with rust!

## Preview
![preview](https://media.discordapp.net/attachments/705201939708772406/1010020825811910666/unknown.png)

## Config
rustfetch configuration is located at ~/.config/rustfetch

### Sample config
```sh
User: `user`
Uptime: `uptime`
Shell: `shell`
```  
rustfetch will automatically replace user, uptime, and shell with their appropriate values.

### Available modules:
\`user\`: Username  
\`distro\`: Distro name
\`uptime\`: System uptime  
\`shell\`: Shell  
\`kernel\`: Kernel version  
\`free\`: Free memory  

## Credits
[nitch](https://github.com/unxsh/nitch) - ASCII art  
[freshfetch](https://github.com/K4rakara/freshfetch) - Inspiration
