# ESSCLI

`esscli` is a Rust command-line utility for interacting with the serverless Elastic Cloud REST API, and (to a lesser extent)
the currently-released API for "stateful" ESS deployments.

This is a development-focused tool, with features designed to accomidate the fact that the serverless api is pre-release and undergoing active breaking changes.

## Getting Started 

To create a new config file, run:

```bash
esscli setup
```

Create a serverless deployment:

```bash
esscli sl create -w -r test-region-default 
```

By default, this will place a new config file and deployment template at `~/.config/ess`.

### REST auth

`esscli` expects a key file, by default located at `~/.config/ess/api_key.txt`. This is the same key file used by the integration test framework in elastic-agent. If you're used to running integration tests in elastic-agent, you're ready to go. 
If not, run `mage integration:auth` from the elastic-agent repo, or manually crate a key and place it in the specifie keyfile via the [ESS API key management page](https://console.qa.cld.elstc.co/account/keys).

### Overrides

For added flexibility, the `esscli`'s config file uses an override system, allowing for stateful and serverless commands to use different URLs and URL paths. By default, the `esscli`'s config override section looks like this:

```toml
[defaults]
url="https://console.qa.cld.elstc.co"
base_path="/api/v1/"

[stateful_override]

[serverless_override]
region="aws-eu-west-1"
url="https://global.qa.cld.elstc.co"
base_path="/api/v1/serverless/"
```

Using this command, any serverless commands (`esscli sl ...`) will use the config values set in `serverless_override`,
while any stateful commands (`ess sf ...`) will fallback to the `default` values. 

Note that since stateful and serverless
currently requre different CSP regions, there is no default `region` config, and one must be supplied in the given `*_override`
config section, or the CLI.

