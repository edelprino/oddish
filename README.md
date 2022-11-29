# Oddish

<img src=".oddish.png" alt="oddish" style="width: 200px;"/>

Oddish is a simple rust program that check your github actions and run a command every time an actions change state

---

Create an `.oddish.toml` file in your root

```
every = 5 #seconds
command = "terminal-notifier -message '{message}' -title 'Oddish'"

[services.github]
token = "xxxxx"
username = "edelprino"
repositories = ["edelprino/oddish"]
```
