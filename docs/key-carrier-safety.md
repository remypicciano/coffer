# Key-carrier safety for everyday use

Key carriers are planned for Coffer format v2. They are not part of the v1 backend.

## The simple rule

If an image, document, or other ordinary file is used as a key carrier, Coffer needs the **exact same file**, byte for byte, to unlock the protected container.

When sending a carrier, choose one of these options:

- **Attach file**
- **Send as document**
- A lossless archive attachment, such as ZIP

Do **not** use:

- an inline image pasted into an email;
- a messaging application's normal photo or gallery button;
- social-media upload and download;
- an editor's Save, Export, Optimize, or Remove metadata command.

Those actions may resize, compress, rotate, recolor, rename internally, remove metadata from, or otherwise rewrite the file. It may look identical to a person while being different to Coffer. A rewritten copy will not work as the key.

## Carrier-only mode

In carrier-only mode, the carrier does not contain a hidden Coffer payload. Nevertheless, possession of the exact carrier is equivalent to possession of the key. Someone who obtains both the carrier and its `.coffer` container can attempt to unlock it.

- Do not send the carrier and container together when channel separation matters.
- Remember that email and messaging providers may retain attachments in mailboxes, backups, notification previews, and synchronized devices.
- Keep at least one byte-identical backup. Losing or changing every copy means losing access permanently.
- Treat an unpublished original as safer than an image already available on social media.

## Carrier plus passphrase mode

The planned stronger mode combines the exact carrier with a passphrase protected by Argon2id. The carrier alone is then insufficient.

- Use a long, unique passphrase or generated multi-word phrase.
- Communicate the passphrase separately from the carrier and container.
- Losing either factor still prevents recovery; Coffer has no backdoor.

## Verifying a transferred copy

The v2 design should show a short fingerprint for local comparison. Matching fingerprints prove that two copies are byte-identical. A fingerprint is for checking copies; it is not a replacement for the carrier and should not be treated as an unlock key.
