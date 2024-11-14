```console
$ rust-pkl-cli eval -f toml examples/simple.pkl
name = "Swallow"

[job]
company = "Nests R Us"
title = "Sr. Nest Maker"
yearsOfExperience = 2


```

```console
$ rust-pkl-cli eval -f toml examples/simple_amending.pkl
[bird]
name = "Swallow"

[bird.job]
company = "Nests R Us"
title = "Sr. Nest Maker"
yearsOfExperience = 2

[parrot]
diet = "Nuts"
name = "Parrot"

[parrot.job]
company = "Nests R Us"
title = "Sr. Nest Maker"
yearsOfExperience = 2


```
