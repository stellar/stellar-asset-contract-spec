# Stellar Asset Contract spec

This repository contains a pre-generated copy of the [SEP-48 Contract Interface Specification] for the [Stellar Asset Contract].

The interface is available in two formats:
- [XDR] in [stellar-asset-spec.xdr](stellar-asset-spec.xdr)
- [XDR-JSON] in [stellar-asset-spec.json](stellar-asset-spec.json)

[XDR]: https://developers.stellar.org/docs/learn/encyclopedia/data-format/xdr
[XDR-JSON]: https://developers.stellar.org/docs/learn/encyclopedia/data-format/xdr-json
[SEP-48 Contract Interface Specification]: https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0048.md
[Stellar Asset Contract]: https://developers.stellar.org/docs/tokens/stellar-asset-contract#:~:text=The%20Stellar%20Asset%20Contract%20allows,to%20use%20Stellar%20assets%20directly.

## Development

Run these commands before committing the changes to generate new JSON and XDR files.

```bash
make gen # generate files
```
