# Introduction - Build

## Prerequisites

You need to have at least NodeJs 22 installed. We recommend to always use the last Long-Term-Support (LTS) version of NodeJs

## Install dependencies

You can install all the dependencies using the following command:

```
npm install
```

If you want to have an exactly reproducible build then you should use instead:

```
npm ci
```

# Run the frontend locally for development purposes

You can run the frontend (without the backend) using the following command:

```
npm run dev
```

You can open the Sveltekit frontend without the backend in a browser using the url: http://localhost:5173/

![Rust - npm - Svelte application browser](./img/svelte_app.png).

# Add test data to the front end for test purposes

Since the frontend is run without backend, you need to let the Svelte endpoints return some mock data without actually connecting to the backend as it does not run.

This mock data is defined in the folder [../src/mockdata/](../src/mockdata/). You can add there more test data.

# Build the frontend

You can buld the frontend using the following command:

```
npm run build
```

# Format the code

Having consistend formatting of code is crucial for its understanding by different developers. We employ here a tool that does this automatically for us.

We use here [Prettier](https://prettier.io/docs/en/) for the typescript code and other files.

The file [../.prettierignore](../.prettierignore) contains all references to folders/files that should not be automatically formatted.

The file [../.prettierrc)](../.prettierrc) allows you to [configure Prettier](https://prettier.io/docs/en/configuration).

You can automatically format all code using the following command:

```
npm run format
```

# Update the dependencies

You can use the following command to update the dependencies:

```
npm update --save
```

Please check the [Svelte Migration Guides](https://svelte.dev/docs/svelte/) when upgrading to another Svelte version.

Make sure you use in your package.json always the latest one. Compare it which one is available on https://www.npmjs.org

# Create Software Bill of Material (SBOM)
The [Software Bill of Material](https://en.wikipedia.org/wiki/Software_supply_chain) (SBOM) is an important machine-readable document that contains all the software and versions that you have used to build the software.

You can create it for the frontend as follows:
```
npm run sbom
```

It will create a file called "frontend-sbom.json" that contains the SBOM for the frontend.