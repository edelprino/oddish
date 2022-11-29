# Oddish
Oddish is a simple rust program that check your github actions and run a command every time an actions change state

### `.oddish.toml`

```
every = 5 #seconds
command = "terminal-notifier -message '{message}' -title 'Oddish'"

[services.github]
token = "xxxxx"
username = "edelprino"
repositories = ["edelprino/oddish"]
```
