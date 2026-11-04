# Authentik blueprints

The Authentik configuration the apps rely on, as [blueprints](https://docs.goauthentik.io/customize/blueprints/):

| File | Content |
|---|---|
| [`groups.yaml`](groups.yaml) | The `Orga`, `Bureau` and `Devs` groups, without their members, and the Kopia credentials of `Devs` |
| [`applications.yaml`](applications.yaml) | The forward auth providers and applications of production and staging, who may open each one, the `X-App-*` headers and the embedded outpost |
| [`google-workspace.yaml`](google-workspace.yaml) | SAML single sign-on into Google Workspace |
| [`google-login.yaml`](google-login.yaml) | "Log in with Google" on the Authentik login page, and Authentik's default sign-up for new Google accounts |
| [`flows.yaml`](flows.yaml) | Login, sign-up (with email confirmation), password recovery, account deletion and the Tangram branding |

## How they are applied

The folder is mounted read-only in the Authentik worker at `/blueprints/tangram`. A production deploy recreates the worker, which applies the blueprints whose content changed. Each blueprint is applied in a single transaction: one that fails leaves the configuration as it was, and shows its error in the Authentik admin, under **Customization → Blueprints**.

The deploy then waits until Authentik has applied the current version of every blueprint (`DEPLOY_AUTHENTIK_BLUEPRINTS` in `production.env`), and rolls back if one fails.

Secrets come from [`infra/authentik/.env`](../sample.env) through `!Env`:

| Variable | Used for |
|---|---|
| `KOPIA_WEB_USERNAME`, `KOPIA_WEB_PASSWORD` | Kopia's web UI login, sent by Authentik so Devs never see it. The same values as in `infra/kopia/.env` |
| `GOOGLE_CLIENT_SECRET` | The Google login's OAuth client secret. Leave it empty to keep the one stored in Authentik |

Staging and production share the one Authentik instance, run by production. A staging deploy does not apply the blueprints, so even changes for the staging applications go live with the next production deploy.

## What stays in the Authentik admin

- users, their passwords and the members of the groups
- the images of the branding (Customization → Files)
- the user settings page, which is Authentik's default

## Rules

- **Change these objects here, not in the admin.** A blueprint sets the fields it lists back to their value each time it is applied. A field it does not list is left as it is.
- **Deleting an entry does not delete the object.** To delete it, keep the entry with `state: absent` until the next deploy, then remove it.
- Objects that come with Authentik (flows, default mappings, certificates…) are referenced with `!Find`, objects of the same blueprint with `!KeyOf`, which must come after the object they point to.
- **Do not change the fields Authentik's own blueprints set on its default objects** (in `/blueprints/default` of the Authentik image). They are applied again when they change in a new Authentik version, and would silently undo ours, which are only applied again when our files change. Use our own object instead, like the sign-up form's own email and password fields.

## Testing a change locally

Apply them to a throwaway Authentik of the same version, loaded with a copy of the production configuration (`ak export_blueprint` on the server), then compare a new export with that copy: only the intended changes should show up.
