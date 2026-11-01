import { createHash } from "node:crypto";
import fs from "node:fs";
import path from "node:path";
import { createClient } from "@hey-api/openapi-ts";
import type { UserConfig } from "@hey-api/openapi-ts";
import {
  addImportsSources,
  defineNuxtModule,
  findPath,
  useLogger,
} from "nuxt/kit";
import { findExports, findTypeExports } from "mlly";

// Generates the API client with @hey-api/openapi-ts and auto-imports its
// exports. Replaces @hey-api/nuxt, which generates the client twice on every
// start (during setup, then again on `app:templates`).
//
// The client is written to `client/` rather than `.nuxt/`, which Nuxt wipes
// on `nuxt prepare` and on config changes. Generation is skipped when neither
// the spec, the config nor the openapi-ts version changed since the last run.

export interface ModuleOptions {
  /** Path to the OpenAPI spec. */
  input: string;
  /** openapi-ts configuration, without `input` and `output`. */
  config?: Omit<UserConfig, "input" | "output">;
  /** Alias to import the generated client from. */
  alias?: string;
}

const HASH_FILE = ".hash";

export default defineNuxtModule<ModuleOptions>({
  meta: {
    name: "hey-api",
    configKey: "heyApi",
  },
  defaults: {
    input: "",
    alias: "#hey-api",
  },
  async setup(options, nuxt) {
    const logger = useLogger("hey-api");
    const output = path.join(nuxt.options.rootDir, "client");

    const config: UserConfig = {
      ...options.config,
      input: options.input,
      output,
      logs: { level: "warn" },
    };

    const openapiTsVersion = JSON.parse(
      fs.readFileSync(
        path.join(
          nuxt.options.rootDir,
          "node_modules/@hey-api/openapi-ts/package.json",
        ),
        "utf-8",
      ),
    ).version;
    const hash = createHash("sha256")
      .update(fs.readFileSync(options.input))
      .update(JSON.stringify(options.config ?? {}))
      .update(openapiTsVersion)
      .digest("hex");

    const hashFile = path.join(output, HASH_FILE);
    const upToDate =
      fs.existsSync(path.join(output, "index.ts")) &&
      fs.existsSync(hashFile) &&
      fs.readFileSync(hashFile, "utf-8") === hash;

    if (upToDate) {
      logger.info("API client up to date");
    } else {
      logger.start(`Generating API client from ${options.input}`);
      await createClient(config);
      fs.writeFileSync(hashFile, hash);
      logger.success("API client generated");
    }

    nuxt.options.alias[options.alias!] = output;

    // Auto-import everything the generated modules export.
    const typeImports = new Set<string>();
    const valueImports = new Set<string>();
    const index = fs.readFileSync(path.join(output, "index.ts"), "utf-8");
    for (const { specifier } of findExports(index)) {
      if (!specifier || !/^\.{1,2}\//.test(specifier)) continue;
      const file = await findPath(path.resolve(output, specifier));
      if (!file) continue;
      const source = fs.readFileSync(file, "utf-8");
      for (const { names } of findTypeExports(source)) {
        names.forEach((name) => typeImports.add(name));
      }
      for (const { names } of findExports(source)) {
        names.forEach((name) => valueImports.add(name));
      }
    }
    addImportsSources({
      from: options.alias!,
      imports: [
        ...[...typeImports].map((name) => ({ name, type: true })),
        ...valueImports,
      ],
    });
  },
});
