# Authentik blueprints

The Authentik configuration the apps rely on, as [blueprints](https://docs.goauthentik.io/customize/blueprints/):

| File | Content |
|---|---|
| [`groups.yaml`](groups.yaml) | The `Orga`, `Bureau` and `Devs` groups, without their members |
| [`applications.yaml`](applications.yaml) | The forward auth providers and applications of production and staging, who may open each one, the `X-App-*` headers and the embedded outpost |
| [`google-workspace.yaml`](google-workspace.yaml) | SAML single sign-on into Google Workspace |

## How they are applied

The folder is mounted read-only in the Authentik worker at `/blueprints/tangram`. A production deploy recreates the worker, which applies the blueprints whose content changed. Each blueprint is applied in a single transaction: one that fails leaves the configuration as it was, and shows its error in the Authentik admin, under **Customization → Blueprints**. A blueprint can also be applied again from there.

Staging and production share the one Authentik instance, run by production. A staging deploy does not apply the blueprints, so even changes for the staging applications go live with the next production deploy.

## What stays in the Authentik admin

- users, their passwords and the members of the groups
- secrets: the Google login source and the Kopia basic auth settings
- the flows, stages, prompts and branding

## Rules

- **Change these objects here, not in the admin.** A blueprint sets the fields it lists back to their value each time it is applied. A field it does not list is left as it is.
- **Deleting an entry does not delete the object.** To delete it, keep the entry with `state: absent` until the next deploy, then remove it.
- Objects that come with Authentik (flows, default mappings, certificates…) are referenced with `!Find`, objects of the same blueprint with `!KeyOf`.

## Testing a change locally

Apply them to a throwaway Authentik of the same version, loaded with a copy of the production configuration (`ak export_blueprint` on the server), then compare a new export with that copy: only the intended changes should show up.
