# Metamask wallet decryption
A very simple application that takes a salt, iv and data from a MetaMask wallet
and tries a wordlist in an attempt to decrypt the wallet.

The name Russet comes from the brown color in the MetaMask fox's ear.

## Invocation

It is very easy to run `russet`:

`russet --salt <salt> --iv <iv> --data <data> [wordlist]`

`russet` will try the `wordlist` first if it's present. If the wordlist fails
the chances of recovering the wallet are now significantly worse.

It will print a result when it sees the start of a json structure: `{`.
