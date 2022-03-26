# copy-mfa

Copy the current Google Authenticator generated number with given GA secret to clipboard from commandline.

## Usage

Pass GA secret as the first argument:

```bash
copy-mfa YOUR_SECRET
```

Or set `MFA_SECRET` as an environment variable and run:

```bash
copy-mfa
```

If the provided secret is valid, it will copy the mfa code to your clipboard and ouput:

```bash
GA number copied: THE_CODE
```

