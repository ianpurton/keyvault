+++
title = "Your First Secret"
date = 2019-11-27
+++

### Step 1: Install Cloak

```bash
$ sudo /bin/sh -c 'wget https://github.com/ianpurton/Cloak/releases/latest/download/cli -O /usr/local/bin/Cloak && chmod +x /usr/local/bin/Cloak'
```

### Step 2: Your first secret

Before you begin, check to verify that no secrets exists at `secret/hello`.

```bash
$ Cloak kv get /secret/hello
No value found at secret/data/hello
```

You can write a new version of the secret with:

```bash$ 
Cloak kv put secret/hello foo=world

Key              Value
---              -----
created_time     2020-09-02T21:40:01.635656Z
deletion_time    n/a
destroyed        false
version          1
```



