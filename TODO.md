# TODO

### Build and Validation system for each and all components

#### Unit level (aka Lint tool)

- Prevents `mkdir` invocations without `-p` argument
- Enforce usage of '' to make Compnents work with paths with spaces
- [TO DISCUSS] `components validate` and corresponding `validate` action so that component can validate itself autonomously.

#### Integration level

- System should be able to build and validate all components in repository
- System should be able to build and validate one component
- System should be able to build and validate reinstallation of different version of a same component

### DRY

- Include templates

### Usability

- Print component's version

### Semantics

- The use of `COMPONENT_SOURCE_PATH` is under discussion
- The use of `COMPONENT_FRAMEWORK_PATH` is under discussion
- The use of `COMPONENT_ARTEFACTS_PATH` is under discussion

