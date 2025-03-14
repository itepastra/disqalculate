A small discord bot that has only a single but powerful command:
```
/calc query:
```
This command passes the query directly to [libQalculate](https://github.com/Qalculate/libqalculate/).

# What can I put in query?

Qalculate is a very powerful and versatile calculator which has many functions.
I won't list them here since they have docs themselves [here](https://qalculate.github.io/manual/).

# How to build and run

It's important to compile libqalculate with the flag `--disable-insecure` to reduce the surface for rce and leaking
sensitive information, the [flake](flake.nix) does this automatically in both the package and the devshell.
After you built the bot it's adviced to run is in a secure place where you add `DISCORD_TOKEN` to the environment.

