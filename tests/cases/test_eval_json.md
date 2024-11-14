```console
$ rust-pkl-cli eval -f json not_existing.pkl
? 1
Error: Failed to read file 'not_existing.pkl'

Caused by:
    No such file or directory (os error 2)

```

```console
$ rust-pkl-cli eval -f json examples/simple.pkl
{
  "job": {
    "company": "Nests R Us",
    "title": "Sr. Nest Maker",
    "yearsOfExperience": 2
  },
  "name": "Swallow"
}

```

```console
$ rust-pkl-cli eval -f json examples/simple_amending.pkl
{
  "bird": {
    "job": {
      "company": "Nests R Us",
      "title": "Sr. Nest Maker",
      "yearsOfExperience": 2
    },
    "name": "Swallow"
  },
  "parrot": {
    "diet": "Nuts",
    "job": {
      "company": "Nests R Us",
      "title": "Sr. Nest Maker",
      "yearsOfExperience": 2
    },
    "name": "Parrot"
  }
}

```
