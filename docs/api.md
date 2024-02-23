# GravitLauncher WebSocket API

### This file describes API for Gravit Launcher LaunchServer WebSocket API

#### We assume that all requests are made to wss://launchserver.myminecraftproject.org/api

#### Instead of endpoint value of "type" field is used

### `/getAvailabilityAuth`

Returns all available auth types for client. No extra params required and ignored if sent

Request:

```json
{
  "type": "getAvailabilityAuth"
}
```

Response:

```json
{
  "list": [
    {
      "details": [
        {
          "type": "password"
        }
      ],
      "name": "std",
      "displayName": "Default",
      "visible": true
    }
  ],
  "features": 1,
  "type": "getAvailabilityAuth"
}
```

```json
{
  "passwordEncryptKey": "32655682f8e21b3379a158b4f9a822e3",
  "runtimeEncryptKey": "c97f9c5b06f9590d",
  "unlockSecret": "e1e35db6c8890fc4bdd5be4aa13c8680",
  "registerApiKey": "e26e00b2a39238a1697664993d3ef05b",
  "clientCheckSecret": "dca0d4aa547bd398e91ac2a5665b2a1c",
  "buildNumber": 63
}
```

### `/auth`

Main request to authorise in launch server

Data to send depends on password encryption type and auth type you want to use

Let's see how to encrypt params based on `auth_id` and `password.type`:

- `auth_id = std`
    - `password.type = plain` - password sent "as is", without any extra encryption. Not very safety
    - `password.type = aes` - password encrypted with AES, see [encryption#AES](./encryption.md#aes)

Request:

```json
{
  "type": "auth",
  "authType": "CLIENT",
  "login": "username",
  "auth_id": "std",
  "password": {
    "password": "password",
    "type": "plain"
  },
  "getSession": true
}
```

Response:

```json
{
  "type": "auth"
}
```