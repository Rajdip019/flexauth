
# Session Management

The session management is done in such a way that you can track each and every user session while giving users long-lived sessions on multiple devices. Also, any detected miscellaneous activity leads to revoking the user session. 


## Tokens issues for a session

We are using multiple tokens for a **session**.

### Hashing Algorithms used.
- `Session ID`: Each session of a new device/browser makes a new user session and is encrypted by the user `DEK`. For more info. Session ID id is the main session Identifier - [User Data Protection](https://github.com/Rajdip019/in-house-auth/blob/main/docs/backend/user-data-protection.md)
- `ID Token`: Holds the identity of the user. An ID token is lived for 1 hour.
- `Refresh Token`: Holds the capability to refresh the session. A refresh token lives for 45 days. Although the refresh token life is shorted by the ID Token as on refresh token can refresh only one session it is paired with.


## Verify Session

While verifying a session from the `session/verify` route. We check if the ID Token is valid and not expired. If both are satisfied it returns user data which you can store in a user state. 

## Refresh Session

In the refresh session first, we ensure that the `ID Token` is already expired, the `Refresh Token` is not expired, and the `ID Token`, `Refresh Token`, and `Session ID` is paired in the same session.

Next, we validate if the user agent for the session is the same or not. If not then we revoke the session and mail the user for malicious activity.

If all goes good then we issue a new pair of `ID Token` and `Refresh Token` and send it back to the user.


## Brute Force protection for Password

We maintain a consecutive failed attempted sign-in count and block the user for some time based on that and also send an email to the user about that and give the device info as well as which device is trying to do this.

- 5 consecutive wrong passwords - 180 seconds block
- 10 consecutive wrong passwords - 600 seconds block
- 15 consecutive wrong passwords - 3600 seconds block

For now, there is no rate limiting by the server itself we highly recommend you do that by using an external service. We will soon implement that natively as well. 

## More malicious activity protection
- If a refresh session is asked and the `ID Token`, `Refresh Token`, and `Session ID` are not paired together we revoke the token immediately. Like if a wrong Refresh token or Session ID is passed for a session ID the session gets blocked.

- We also have a revoke-all-session endpoint for users that can be used to sign out from all devices/browsers of the user.

## Feedback

If you have any feedback, please raise an issue or start a discussion. Thank you.
