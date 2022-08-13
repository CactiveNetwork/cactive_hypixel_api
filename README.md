# Unofficial Hypixel API by CactiveNetwork

This is an implementation of the API in Rust, however you're welcome to request data directly.

The method of collecting data is **private**, hence why a key system is required.

### Disclaimer:

- We are not associated nor should we be considered affiliated with Hypixel.
- A valid API key is required to successfully request player, nickname, ban and other data from this API. You can request an API key running `-ticket open` [in this discord](https://discord.gg/NeqVuSy) providing you meet criteria.
---

## Error Types

In the case that your request is invalid, maintenance is done on the API, or the tunnel collecting data is blocked or locked out, you will receive an error, which you should look out for.

This wrapper adds an `internal` field to the object in your array of errors, which will be `true` if the error is internal to your device (unable to send request or parse JSON), otherwise false.

- `no-authentication` - You didn't provide the relevant authentication information (key field).
- `no-identifier` - You didn't provide the relevant username, id, or such required identification field.
- `no-filter` - You didn't provide the relevant filter field to your request.
- `invalid-authentication` - The authentication information provided in the request was invalid.
- `invalid-filter` - The filter provided in the request was invalid.
- `invalid-endpoint` - The endpoint provided in the request was invalid.
- `tunnel-blocked` - Unable to interface with the required relevant request data.
- `hypixel-maintenance` - Unable to complete because one of the data sources are offline.
- `rate-limit-blocked` - You are being rate limited for sending too many valid requests.
- `failed-api-request` **NODE CLIENT** - The client failed to send a valid request to the server.
- `unexpected-error` - An unexpected error occurred on the API server.

---

## Rate Limits

- Standard requests are limited to `10 requests per minute`.
- Smart-cached requests (semi-accurate data) are limited to `30 request per minute`.

You are expected to respect these limits, and if you do not, you will be blocked from the API.