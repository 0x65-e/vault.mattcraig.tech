# vault.mattcraig.tech: Serverless Temporary Filesharing in Rust!

[![Build Status](https://img.shields.io/github/actions/workflow/status/0x65-e/vault.mattcraig.tech/workers_deploy.yml?branch=master&logo=github&style=flat-square)](https://github.com/0x65-e/vault.mattcraig.tech/actions/workflows/workers_deploy.yml) [![License](https://img.shields.io/github/license/0x65-e/vault.mattcraig.tech?logo=opensourceinitiative&style=flat-square)](https://opensource.org/license/mit/)

## About

This website is written for [Cloudflare Workers](https://workers.cloudflare.com/) using Rust and the [workers-rs](https://github.com/cloudflare/workers-rs) crate. Cloudflare Workers is a serverless, "function as a service" (FaaS) platform that runs across distributed data centers.

It is a toy filesharing service that permits temporary uploads and downloads of PSK-protected files using [Cloudflare R2](https://www.cloudflare.com/products/r2/), an S3-compatible globally distributed object storage for Cloudflare Workers. **It makes no guarantee or warranty, express or implied, of privacy or security.**

## Usage

With `wrangler` CLI, you can build, test, and deploy to Workers with the following commands: 

```bash
# compiles project to WebAssembly and will warn of any issues
wrangler build 

# runs Worker in an ideal development workflow (with a local server, file watcher & more)
wrangler dev

# deploys Worker globally to Cloudflare
wrangler publish
```

You will need to generate your own R2 bucket and replace the values in [wrangler.toml](wrangler.toml).

```bash
# creates an R2 bucket
wrangler r2 bucket create "FILES"

# creates a bucket for the staging environment
wrangler r2 bucket create "FILES_STAGING"
```

You can choose a name other than `FILES` for your bucket, but be sure to update the R2 access in [libs.rs](src/lib.rs).

You may also want to change the name of your worker in [wrangler.toml](wrangler.toml).

### Continuous Deployment

This repository includes a [Github Workflow](.github/workflows/workers_deploy.yml) that automatically publishes the latest version of the `master` branch on every push.

For CD to work properly with the wrangler CLI, you must set up repository secrets named `CF_ACCOUNT_ID` and `CF_API_TOKEN` with your Cloudflare account ID and API token as described in the [wrangler system environment variables guide](https://developers.cloudflare.com/workers/wrangler/system-environment-variables/).
