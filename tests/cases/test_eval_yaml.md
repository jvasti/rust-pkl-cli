```console
$ rust-pkl-cli eval -f yaml examples/simple.pkl
job:
  company: Nests R Us
  title: Sr. Nest Maker
  yearsOfExperience: 2
name: Swallow


```

```console
$ rust-pkl-cli eval -f yaml examples/simple_amending.pkl
bird:
  job:
    company: Nests R Us
    title: Sr. Nest Maker
    yearsOfExperience: 2
  name: Swallow
parrot:
  diet: Nuts
  job:
    company: Nests R Us
    title: Sr. Nest Maker
    yearsOfExperience: 2
  name: Parrot


```
